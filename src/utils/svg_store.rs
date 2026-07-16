// svg_store.rs
//
// This module provides `SvgStore`, a lightweight, thread-safe, in-memory
// registry of SVG assets that bridges our runtime-generated SVG bytes (produced
// by Typst/resvg) with GPUI's asset pipeline.
//
// ── Why a custom AssetSource? ────────────────────────────────────────────────
// GPUI renders SVGs through `Window::paint_svg`, which accepts a *path string*
// rather than raw bytes. Internally, GPUI calls `AssetSource::load(path)` on
// whatever source was registered at app startup, reads the bytes, rasterizes
// them into its sprite atlas, and caches the result by path. This design lets
// GPUI deduplicate rasterization work across frames: if the same path is
// painted twice, the atlas hit is free.
//
// Because our SVGs are generated at runtime (not bundled on disk), we need a
// custom `AssetSource` that can serve bytes by an in-memory key.  `SvgStore`
// fills that role: callers insert bytes, receive back a stable path key, and
// later hand that key to `paint_svg`.
//
// ── Why Arc<RwLock<…>>? ──────────────────────────────────────────────────────
// GPUI may call `AssetSource::load` from the render thread, while SVGs are
// inserted from wherever formula rendering happens (potentially a different
// task or thread).  `Arc<RwLock<…>>` lets multiple readers (GPUI loading
// assets) proceed concurrently, while writers (inserting new SVGs) get
// exclusive access only when needed.  `Arc` makes cheap clone-sharing safe
// across thread boundaries — every `SvgStore` clone (e.g. one held by the app,
// one registered with GPUI) points at the same backing map.

use std::{
    borrow::Cow,
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    sync::{Arc, RwLock},
};
use anyhow::Result;
use gpui::{AssetSource, SharedString};

/// Thread-safe, in-memory store for SVG assets, registered with GPUI as an
/// [`AssetSource`].
///
/// # Lifecycle
///
/// 1. At app startup, create one `SvgStore` and pass a clone to
///    `AppContext::set_asset_source` (or equivalent).  Both the app logic and
///    GPUI now share the same backing map via `Arc`.
/// 2. When a LaTeX/Typst formula is rendered to an SVG, call [`SvgStore::insert`]
///    with the raw SVG bytes.  You receive back a [`SharedString`] path key.
/// 3. Pass that path key to `Window::paint_svg`.  GPUI will call
///    [`AssetSource::load`] with the same key to fetch the bytes from this
///    store, rasterize them once, and cache the sprite in its atlas.
///
/// # Deduplication
///
/// [`SvgStore::insert`] derives the path key from a *content hash* of the
/// bytes.  Identical SVG content (e.g. the same formula appearing multiple
/// times in a document) always maps to the same key, so GPUI's atlas only
/// rasterizes it once across the entire app lifetime — regardless of how many
/// elements render it.
///
/// # Cloning
///
/// Cloning is O(1) and cheap: the `Arc` reference count is simply incremented.
/// All clones share the same `HashMap`.
#[derive(Clone, Default)]
pub struct SvgStore {
    /// The actual SVG bytes, keyed by content-hash path strings.
    ///
    /// `Arc` allows multiple owners (app code + GPUI's asset source) to share
    /// the map without copying.  `RwLock` permits concurrent reads from GPUI's
    /// render path while serializing the rare write (a new SVG insertion).
    svgs: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl SvgStore {
    /// Create an empty `SvgStore`.
    ///
    /// Equivalent to `SvgStore::default()` — provided for ergonomics so
    /// callers don't need to import `Default`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert SVG bytes into the store and return a stable path key.
    ///
    /// # Key derivation
    ///
    /// The path is derived from a 64-bit [`DefaultHasher`] digest of the raw
    /// bytes, formatted as:
    ///
    /// ```/dev/null/example.txt#L1-1
    /// latex-svg/<16 hex digits>.svg
    /// ```
    ///
    /// Using a content hash (rather than, say, an incrementing counter)
    /// guarantees that *identical SVG content always produces the same key*.
    /// This is the property GPUI relies on for sprite-atlas deduplication:
    /// if the same formula is painted in 50 places, `paint_svg` will call
    /// `load` for the same path 50 times, but GPUI only rasterizes it once
    /// because the path (== cache key) is stable.
    ///
    /// # Idempotence
    ///
    /// `entry(…).or_insert(…)` means inserting the same bytes a second time
    /// is a no-op: the existing entry is returned unchanged.  This keeps the
    /// map compact even if formula rendering is invoked redundantly.
    ///
    /// # Returns
    ///
    /// A [`SharedString`] (reference-counted, cheaply cloneable) containing
    /// the path key.  Store this and pass it to `paint_svg`.
    pub fn insert(&self, bytes: Vec<u8>) -> SharedString {
        // Hash the raw bytes to produce a deterministic, content-addressed key.
        // `DefaultHasher` is fast and sufficient here — we don't need
        // cryptographic strength, only collision resistance for typical SVG
        // payloads within a single session.
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);

        // Format as a virtual file path so GPUI's asset system treats it like
        // any other named asset file.  The ".svg" extension is required —
        // GPUI inspects the extension to know which codec to use.
        let path = format!("latex-svg/{:016x}.svg", hasher.finish());

        // Acquire a write lock only long enough to insert the entry.
        // `or_insert` skips the insertion if the key already exists, so
        // repeated calls for the same formula don't duplicate bytes.
        self.svgs
            .write()
            .unwrap()
            .entry(path.clone())
            .or_insert(bytes);

        // Wrap in SharedString so callers get a cheap-to-clone handle instead
        // of an owned String.  SharedString is what paint_svg and other GPUI
        // APIs expect for path arguments.
        SharedString::from(path)
    }
}

/// Implementation of GPUI's [`AssetSource`] trait, which makes `SvgStore`
/// usable as the app's registered asset provider.
///
/// GPUI calls these methods internally — you never need to call them directly.
/// Register this store once with `AppContext::set_asset_source` and then just
/// use path keys from [`SvgStore::insert`] with `paint_svg`.
impl AssetSource for SvgStore {
    /// Return the bytes for a previously-inserted SVG, or `None` if the path
    /// is unknown.
    ///
    /// # Why `Cow::Owned`?
    ///
    /// The trait signature requires `Cow<'static, [u8]>`.  We can't return a
    /// `Cow::Borrowed` because the borrow would need to outlive the lock guard
    /// (which is dropped at the end of this call).  `Cow::Owned` clones the
    /// bytes, giving GPUI full ownership so it can hold them as long as needed
    /// for rasterization without keeping our lock held.
    ///
    /// The clone cost is paid at most once per unique SVG: after the first
    /// rasterization, GPUI caches the sprite in its atlas and no longer calls
    /// `load` for the same path.
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(self
            .svgs
            // Read lock — multiple GPUI render threads can call load()
            // concurrently without blocking each other.
            .read()
            .unwrap()
            .get(path)
            // Clone the Vec<u8> into an Owned Cow so we can release the lock
            // before returning.  Borrowing through the lock guard is not
            // possible because the guard's lifetime doesn't extend to 'static.
            .map(|b| Cow::Owned(b.clone())))
    }

    /// List assets under a virtual directory prefix.
    ///
    /// GPUI calls this for tooling (e.g. asset explorers) but never for
    /// rendering.  Returning an empty vec is safe — our assets are addressed
    /// directly by path, not discovered by directory listing.
    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

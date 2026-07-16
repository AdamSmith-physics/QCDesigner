//! Converts a LaTeX math string into an SVG image suitable for display in GPUI.
//!
//! # Pipeline
//!
//! ```text
//! LaTeX string
//!     │
//!     ▼  tex2typst_rs::tex2typst()
//! Typst math string
//!     │
//!     ▼  Typst compiler (StubWorld)
//! Typst PagedDocument
//!     │
//!     ▼  typst_svg::svg()
//! SVG string  ──►  LatexSvg { svg_bytes, width_pt, height_pt }
//! ```
//!
//! The caller (in `svg_store`) stores the bytes under a unique path key and
//! later hands that key to GPUI's `paint_svg`, which re-reads the bytes
//! through the `AssetSource` trait.  See `svg_store.rs` for why the
//! path-key indirection is required.

use std::path::PathBuf;
use std::sync::OnceLock;
use tex2typst_rs::tex2typst;
use time::OffsetDateTime;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

// ---------------------------------------------------------------------------
// Public output type
// ---------------------------------------------------------------------------

/// The product of a successful LaTeX → SVG conversion.
///
/// `svg_bytes` is UTF-8–encoded SVG text ready to be stored in `SvgStore`.
/// `width_pt` / `height_pt` are the *intrinsic* dimensions reported by Typst
/// in its `width="…pt"` / `height="…pt"` SVG attributes.
///
/// Why pt == logical pixels in GPUI
/// ---------------------------------
/// Typst emits dimensions in typographic points (1 pt = 1/72 in).  GPUI's
/// `paint_svg` function works in *logical pixels*, and on a standard 72 dpi
/// mapping 1 pt ≡ 1 px.  On high-DPI (Retina) displays GPUI itself scales
/// the logical pixel rectangle up to physical pixels, so the caller never
/// needs to multiply by a device-pixel ratio — just pass `width_pt` directly
/// to `px(width_pt)`.
pub struct LatexSvg {
    /// Raw UTF-8 bytes of the generated SVG document.
    pub svg_bytes: Vec<u8>,

    /// Intrinsic SVG width expressed in typographic points.
    /// Treat this value as logical pixels when constructing a GPUI `Pixels`.
    pub width_pt: f32,

    /// Intrinsic SVG height expressed in typographic points.
    /// Treat this value as logical pixels when constructing a GPUI `Pixels`.
    pub height_pt: f32,
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Convert a LaTeX math expression to an SVG image.
///
/// # Arguments
///
/// * `latex`        – Raw LaTeX math source, e.g. `r"\frac{a}{b}"`.
///                    The surrounding `$…$` delimiters must *not* be included;
///                    they are added by this function when building the Typst
///                    document.
/// * `font_size_px` – Desired rendered font size in logical pixels / pt.
///                    This is forwarded verbatim to Typst's `#set text(size:)`.
///
/// # Errors
///
/// Returns a human-readable `String` if:
/// * `tex2typst` cannot translate the LaTeX snippet, or
/// * the Typst compiler emits one or more diagnostics (syntax / type errors).
///
/// # DPI scaling
///
/// GPUI's `paint_svg` handles DPI-aware up-scaling automatically.  The caller
/// only needs to reserve a logical-pixel rectangle whose size matches the
/// returned `width_pt` / `height_pt`.
pub fn latex_to_svg(latex: &str, font_size_px: f32) -> Result<LatexSvg, String> {
    // Step 1: Translate LaTeX syntax to Typst math syntax.
    // tex2typst is a best-effort converter; obscure LaTeX macros may fail here.
    let typst_math = tex2typst(latex).map_err(|e| e.to_string())?;

    // Step 2: Wrap the translated math in a minimal Typst document.
    //
    // Page settings:
    //   width/height: auto  → the page shrinks to fit the content exactly,
    //                          so the resulting SVG has tight natural dimensions.
    //   margin: 1pt          → a tiny margin prevents the ink from touching the
    //                          SVG border, which would clip descenders / accents.
    //   fill: none           → transparent background; the GPUI tint colour is
    //                          applied at paint time instead.
    //
    // `$ … $` is Typst's inline-math delimiter (double `$$` for display math).
    // We use inline here so single-line equations stay on one line; the auto
    // page size adjusts regardless.
    let document_source = format!(
        r#"
        #set page(width: auto, height: auto, margin: 1pt, fill: none)
        #set text(size: {font_size_px}pt)
        $ {} $
        "#,
        typst_math
    );

    // Step 3: Compile the Typst source through our minimal world stub.
    // `StubWorld` satisfies Typst's `World` trait with the bare minimum needed
    // to compile a self-contained math snippet (one font, no file system).
    let world = StubWorld::new(document_source);
    let result = typst::compile::<PagedDocument>(&world);

    // `result.output` is `Err` when *any* Typst diagnostic is an error.
    // Warnings are present in `result.warnings` but we ignore them here.
    let document = result
        .output
        .map_err(|diags| format!("Typst compilation error: {:?}", diags))?;

    // Step 4: Render the first (and only) page to an SVG string.
    // `typst_svg::svg` is Typst's own SVG back-end; it embeds the font glyphs
    // as paths so the output is fully self-contained (no external font needed
    // by the SVG renderer).
    let svg_str = typst_svg::svg(&document.pages[0]);

    // Step 5: Extract the intrinsic dimensions from the SVG attributes so the
    // caller can size the GPUI element correctly without parsing the SVG again.
    // Fall back to a sensible default if the attributes are missing/malformed.
    let (width_pt, height_pt) = parse_svg_dimensions(&svg_str).unwrap_or((100.0, 30.0));

    Ok(LatexSvg {
        svg_bytes: svg_str.into_bytes(),
        width_pt,
        height_pt,
    })
}

// ---------------------------------------------------------------------------
// SVG dimension helpers
// ---------------------------------------------------------------------------

/// Parse both `width="…pt"` and `height="…pt"` from the SVG root element.
/// Returns `None` if either attribute is absent or non-numeric.
fn parse_svg_dimensions(svg: &str) -> Option<(f32, f32)> {
    // Both must succeed; if either is missing we fall back to the caller's default.
    Some((parse_pt_attr(svg, "width")?, parse_pt_attr(svg, "height")?))
}

/// Extract the numeric pt value from a single SVG attribute of the form
/// `attr="123.45pt"`.
///
/// The function does a simple string search rather than a full XML parse
/// because:
///   1. The SVG is machine-generated by Typst and has a stable format.
///   2. A full XML parse would add a dependency for two attribute reads.
fn parse_pt_attr(svg: &str, attr: &str) -> Option<f32> {
    // Build the opening token, e.g. `width="`.
    let needle = format!("{}=\"", attr);
    // Find where the value starts (just after the opening quote).
    let start = svg.find(&needle)? + needle.len();
    // Find the closing quote relative to `start` and convert to an absolute index.
    let end = start + svg[start..].find('"')?;
    // Strip the "pt" suffix and parse the remaining digits.
    svg[start..end].strip_suffix("pt")?.parse().ok()
}

// ---------------------------------------------------------------------------
// Minimal Typst World implementation
// ---------------------------------------------------------------------------

/// A bare-minimum implementation of Typst's [`World`] trait.
///
/// Typst requires a `World` to supply:
/// * the standard library,
/// * a font book (index of available fonts),
/// * the fonts themselves,
/// * the main source file to compile, and
/// * an optional current date.
///
/// For our use case (compiling a single self-contained math snippet) we do
/// not need a real file system, package registry, or multi-file project.
/// Any request for an *external* file is answered with `FileError::NotFound`.
struct StubWorld {
    /// The Typst standard library (built-in functions, types, modules).
    /// `LazyHash` defers hashing until Typst actually needs the hash, which
    /// avoids paying the hashing cost when the library is constructed.
    library: LazyHash<Library>,

    /// Metadata index of all fonts available to this world.
    /// Typst uses the book to select a font family by name/style; it then
    /// requests the actual `Font` object via `font(index)`.
    book: LazyHash<FontBook>,

    /// The actual loaded font objects, indexed the same way as `book`.
    /// We embed a single math font (NewCMMath) which is sufficient to render
    /// all standard mathematical symbols.
    fonts: Vec<Font>,

    /// The single source file to compile.
    /// `Source::detached` creates a synthetic file with no real path on disk.
    source: Source,

    /// Current wall-clock time, captured once at world-construction time.
    /// Typst exposes `datetime.today()` to documents; our stub uses a real UTC
    /// timestamp so that Typst's date/time functions don't error out.
    time: OffsetDateTime,
}

impl StubWorld {
    /// Create a new `StubWorld` that will compile `source_text`.
    ///
    /// Font loading is performed only **once** per process, regardless of how
    /// many equations are rendered.  `OnceLock` ensures that the expensive
    /// `include_bytes!` (which bakes the font into the binary) + font parsing
    /// happen at most once, and that subsequent calls simply clone the
    /// already-initialised `Arc`-like handles inside `LazyHash` / `Vec`.
    fn new(source_text: String) -> Self {
        // `OnceLock` is the std equivalent of `lazy_static!` for a single
        // value.  Because `LazyHash<FontBook>` and `Font` are `Clone + Send +
        // Sync`, storing them in a `'static` OnceLock is safe even when
        // `latex_to_svg` is called from multiple threads.
        static FONTS: OnceLock<(LazyHash<FontBook>, Vec<Font>)> = OnceLock::new();

        let (book, fonts) = FONTS
            .get_or_init(|| {
                // Embed the NewCMMath font directly into the binary at compile
                // time.  This makes the executable fully self-contained — no
                // font installation required on the end-user's machine.
                let font_data: &'static [u8] =
                    include_bytes!("../assets/fonts/NewCMMath-Regular.otf");

                // `Font::new` parses the binary font data.  Index 0 selects
                // the first (and only) face in a single-face OTF file.
                let font = Font::new(Bytes::new(font_data), 0)
                    .expect("Failed to load embedded font");

                // Build the font book by registering the font's metadata.
                // The book maps family/style names to numeric indices; Typst
                // calls `font(index)` to retrieve the matching `Font` object.
                let mut book = FontBook::new();
                book.push(font.info().clone());

                (LazyHash::new(book), vec![font])
            })
            // Clone is cheap: `LazyHash<FontBook>` wraps an `Arc`, and `Vec<Font>`
            // clones the `Arc`-backed `Font` handles, not the raw glyph data.
            .clone();

        Self {
            // `Library::default()` builds the full Typst standard library.
            // Wrapping it in `LazyHash` defers the (relatively expensive)
            // hash computation until Typst's incremental-compilation layer
            // actually needs to invalidate a cached result.
            library: LazyHash::new(Library::default()),
            book,
            fonts,
            // `Source::detached` creates a virtual file with an auto-generated
            // `FileId`.  Because our document never `#import`s anything, we
            // never need a real on-disk path.
            source: Source::detached(source_text),
            // Snapshot the current time once; Typst may query it multiple
            // times during compilation and expects a stable value.
            time: OffsetDateTime::now_utc(),
        }
    }
}

impl World for StubWorld {
    /// Return the Typst standard library.
    fn library(&self) -> &LazyHash<Library> { &self.library }

    /// Return the font metadata index.
    fn book(&self) -> &LazyHash<FontBook> { &self.book }

    /// Return the `FileId` of the entry-point source file.
    fn main(&self) -> FileId { self.source.id() }

    /// Return the source for a given `FileId`.
    ///
    /// Because our world contains exactly one source file, we return it
    /// unconditionally for any id.  A real world would look up a file table.
    fn source(&self, _id: FileId) -> FileResult<Source> { Ok(self.source.clone()) }

    /// Return raw bytes for a file path (used for `#include` / images etc.).
    ///
    /// We never reference external files in the generated Typst source, so
    /// this always returns `NotFound`.  Returning an error here rather than
    /// panicking keeps Typst's error-reporting path intact.
    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(PathBuf::from("missing")))
    }

    /// Return the font at the given index in the font book.
    ///
    /// Typst calls this after selecting a font via `book()`.  We have exactly
    /// one font, so `index` will always be 0 for valid lookups.
    fn font(&self, index: usize) -> Option<Font> { self.fonts.get(index).cloned() }

    /// Return a `Datetime` representing today, optionally shifted by `offset`
    /// hours from UTC.
    ///
    /// `offset: None` means "use UTC".  We ignore the offset for simplicity
    /// because our math documents never call `datetime.today()`, but Typst
    /// may invoke this method during library initialisation.
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::Date(self.time.date()))
    }
}

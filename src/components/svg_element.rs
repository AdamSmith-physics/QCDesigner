// svg_element.rs
//
// This module provides `SvgInlineElement`, a hand-written GPUI [`Element`]
// that renders a single SVG from our `SvgStore` at an exact, intrinsic size.
//
// ── Why a custom Element instead of GPUI's built-in `svg()` helper? ─────────
// GPUI's high-level `svg()` component is designed for *icon-sized* assets
// whose dimensions are driven by CSS-style layout constraints (flex, fixed
// width/height set via builder methods, etc.).  LaTeX formulas have *intrinsic*
// dimensions determined by the typesetter: the element must be exactly as wide
// and tall as the formula output, regardless of available space.
// `request_measured_layout` lets us report those exact logical-pixel dimensions
// to the Taffy layout engine, so the formula slots precisely into inline text
// without stretching or clipping.
//
// ── Why paint_svg and not a raw image? ───────────────────────────────────────
// `paint_svg` rasterizes the SVG into an *alpha-mask* sprite — a single-
// channel bitmap that records coverage (opacity) but no color.  The tint color
// passed to `paint_svg` is then multiplied against the mask at render time.
// This has two benefits:
//   1. Resolution-independent: the mask is rasterized at the physical pixel
//      density of the current display, so formulas are crisp on both Retina
//      and standard monitors without storing multiple bitmaps.
//   2. Cheap theme switching: to change the formula color (e.g. dark-mode
//      inversion), only the tint parameter changes — no re-rasterization.
//
// ── Why store a path key and not the raw bytes? ──────────────────────────────
// `paint_svg` identifies SVGs by *path*, not by bytes.  GPUI's sprite atlas
// uses the path as a cache key: the same path is rasterized at most once per
// display scale, and the resulting sprite is reused across all frames.  If we
// passed bytes directly there would be no caching.  The `SvgStore` (an
// `AssetSource`) maps our runtime-generated path keys back to bytes when GPUI
// first needs to rasterize an SVG.

use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, Hsla, InspectorElementId, IntoElement,
    LayoutId, Pixels, SharedString, Size, Style, TransformationMatrix, Window,
};

/// A custom GPUI element that renders an in-memory SVG using `paint_svg`.
///
/// The SVG bytes live in an `SvgStore` (an `AssetSource`). GPUI's `paint_svg`
/// renders the SVG as a monochrome alpha-mask sprite — vector-crisp at any
/// DPI, tinted with the provided color.  No manual rasterization is required.
///
/// # Usage
///
/// ```/dev/null/usage_example.rs#L1-6
/// let path = svg_store.insert(svg_bytes);
/// SvgInlineElement::new(path, px(120.0), px(18.0), black())
///     .into_any_element()
/// ```
///
/// # Layout contract
///
/// The element always occupies exactly `width × height` logical pixels.  It
/// ignores any external constraints (no flex grow/shrink).  This is intentional:
/// the dimensions come from the typesetter and must be preserved to avoid
/// distorting the formula.
pub struct SvgInlineElement {
    /// Cache key in the `SvgStore`, e.g. `"latex-svg/deadbeef01234567.svg"`.
    ///
    /// This string is passed verbatim to `paint_svg` and used by GPUI as the
    /// sprite-atlas cache key.  It must match what was returned by
    /// `SvgStore::insert` for the same SVG bytes.
    path: SharedString,

    /// Intrinsic width in logical (device-independent) pixels.
    ///
    /// Set to the point-width reported by Typst after rendering the formula.
    /// Because GPUI's `Pixels` type represents logical pixels (not physical
    /// device pixels), this value is display-scale–agnostic: GPUI scales it
    /// to physical pixels internally when rasterizing the SVG sprite.
    width: Pixels,

    /// Intrinsic height in logical (device-independent) pixels.
    ///
    /// Same semantics as `width` — sourced from Typst's typeset output.
    height: Pixels,

    /// RGBA color tint applied to the SVG alpha mask at paint time.
    ///
    /// `paint_svg` treats the SVG as a single-channel alpha mask and multiplies
    /// this color against it.  Use `gpui::black()` for traditional dark-on-
    /// white math, or any `Hsla` value for themed rendering (e.g. adapting to
    /// a dark-mode background without re-rasterizing the SVG).
    color: Hsla,
}

impl SvgInlineElement {
    /// Construct a new `SvgInlineElement`.
    ///
    /// # Parameters
    ///
    /// - `path`  — The path key returned by `SvgStore::insert` for this SVG.
    ///             Accepts anything that converts `Into<SharedString>` (a
    ///             `SharedString`, `&str`, `String`, etc.).
    /// - `width` — Logical pixel width of the rendered formula.
    /// - `height`— Logical pixel height of the rendered formula.
    /// - `color` — Tint color for the alpha-mask sprite.  Accepts anything
    ///             `Into<Hsla>` — `gpui::black()`, `gpui::white()`, etc.
    pub fn new(
        path: impl Into<SharedString>,
        width: Pixels,
        height: Pixels,
        color: impl Into<Hsla>,
    ) -> Self {
        Self {
            path: path.into(),
            width,
            height,
            color: color.into(),
        }
    }
}

/// `IntoElement` is GPUI's "type-erased element factory" trait.
///
/// Implementing it for `SvgInlineElement` (with `Element = Self`) means the
/// struct is *already* its own element — no conversion or wrapping is needed.
/// This satisfies GPUI's `child()` / `into_any_element()` call sites that
/// accept `impl IntoElement`.
impl IntoElement for SvgInlineElement {
    type Element = Self;

    /// Return self unchanged — no allocation or conversion required.
    fn into_element(self) -> Self {
        self
    }
}

/// Core GPUI rendering protocol for `SvgInlineElement`.
///
/// GPUI's element lifecycle has three phases, called in order every frame:
///
/// 1. **`request_layout`** — Tell the Taffy layout engine how much space this
///    element needs.  We report our exact intrinsic size.
/// 2. **`prepaint`** — Perform any hit-test registration or pre-draw work
///    using the final bounds from layout.  We have nothing to do here.
/// 3. **`paint`** — Issue actual draw calls.  We call `paint_svg`.
impl Element for SvgInlineElement {
    /// No per-frame layout state is threaded between phases for this element.
    type RequestLayoutState = ();

    /// No per-frame prepaint state is threaded into `paint` for this element.
    type PrepaintState = ();

    /// Return `None` — this element is purely visual and stateless.
    ///
    /// GPUI uses element IDs to track stateful elements across frames (e.g.
    /// inputs, scroll areas).  An SVG formula has no interactive state, so
    /// an ID is not needed.
    fn id(&self) -> Option<ElementId> {
        None
    }

    /// Return `None` — source location tracking is only needed for elements
    /// that participate in GPUI's inspector / dev-tools.  A pure visual
    /// element doesn't need it.
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    /// Phase 1 — register this element's size with the layout engine.
    ///
    /// We use [`Window::request_measured_layout`] rather than the simpler
    /// `request_layout` because we need to supply our *own* concrete size
    /// instead of letting Taffy compute one from style properties.
    ///
    /// The closure receives `(known_size, available_space, window, cx)` in
    /// case the element wants to respond to constraints (e.g. a text block
    /// that wraps).  We ignore both arguments and always return our fixed
    /// intrinsic dimensions — formulas must not be stretched.
    ///
    /// Both `width` and `height` are captured by value (not by reference)
    /// because the closure may outlive this method's stack frame.
    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        _cx: &mut App,
    ) -> (LayoutId, ()) {
        // Capture by value: the closure handed to request_measured_layout is
        // stored until layout is resolved, so we can't borrow `self` here.
        let (w, h) = (self.width, self.height);

        let layout_id = window.request_measured_layout(
            // An empty Style means "no flex/grid constraints of our own".
            // Taffy will still place us in the flow, but our measured size
            // (returned by the closure below) will win over any flex stretch.
            Style::default(),
            // The measure function is called by Taffy after its first pass to
            // ask "given these constraints, how large do you want to be?".
            // We always answer with our exact typeset dimensions.
            move |_known, _avail, _, _| Size { width: w, height: h },
        );
        (layout_id, ())
    }

    /// Phase 2 — called after layout is resolved, before painting.
    ///
    /// Typical uses: register a hitbox for mouse interaction, stash the final
    /// `Bounds` for use in `paint`, clip children, etc.
    ///
    /// For a purely visual SVG element there is nothing to do: no hit testing,
    /// no children, no state to propagate.  The bounds are passed directly into
    /// `paint` by GPUI, so we don't even need to store them here.
    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _layout: &mut (),
        _window: &mut Window,
        _cx: &mut App,
    ) {
        // No hitbox needed; purely visual.
    }

    /// Phase 3 — issue the actual draw call.
    ///
    /// [`Window::paint_svg`] looks up `self.path` in the registered
    /// `AssetSource` (`SvgStore`), rasterizes the SVG into GPUI's sprite atlas
    /// at the appropriate physical resolution (if not already cached), and
    /// schedules a GPU draw call that composites the alpha-mask sprite tinted
    /// with `self.color` into the `bounds` rectangle.
    ///
    /// # TransformationMatrix::unit()
    ///
    /// We pass the identity matrix — no rotation, skew, or extra scaling.
    /// The SVG is already sized in logical pixels; GPUI handles the HiDPI
    /// scale internally when it rasterizes the atlas sprite.
    ///
    /// # Error handling
    ///
    /// `paint_svg` returns a `Result` that we intentionally discard with `let _`.
    /// Failure (e.g. a missing path) is non-fatal: the formula simply won't
    /// appear, but the app continues running.  In a production context you
    /// would log the error here.
    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,   // final pixel rect assigned by the layout engine
        _layout: &mut (),
        _prepaint: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let _ = window.paint_svg(
            bounds,                        // where to draw (position + size)
            self.path.clone(),             // which SVG to look up in SvgStore
            TransformationMatrix::unit(),  // no extra transform — identity matrix
            self.color,                    // tint color applied to the alpha mask
            cx,
        );
    }
}

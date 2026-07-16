use gpui::*;

use crate::components::SvgInlineElement;
use crate::utils::SvgStore;
use crate::utils::latex_renderer::latex_to_svg;



/// A GPUI component that displays one LaTeX equation as an SVG image.
///
/// Rendering is *eager*: the LaTeX is compiled to SVG immediately in `new` (or
/// whenever the latex string / font size changes) rather than lazily in
/// `render`.  This means `render` is always fast and allocation-free — it just
/// reads from the pre-computed `cached` result.
pub struct LatexLabel {
    /// The raw LaTeX math source currently displayed, e.g. `r"\frac{a}{b}"`.
    latex: SharedString,

    /// Font size in logical pixels (== typographic points at 72 dpi).
    /// Passed to Typst's `#set text(size: …pt)`.
    font_size: f32,

    /// Tint colour applied by GPUI's `paint_svg` to the SVG.
    /// Because the SVG background is transparent (`fill: none`), this becomes
    /// the foreground colour of the rendered glyphs.
    color: Hsla,

    /// Shared handle to the SVG byte store.
    /// `SvgStore` wraps `Arc<RwLock<…>>`, so this clone participates in the
    /// same map that was registered with GPUI as an `AssetSource`.
    svg_store: SvgStore,

    /// Pre-compiled render result.
    ///
    /// * `Ok((key, width_pt, height_pt))` — compilation succeeded.  `key` is
    ///   the path string under which the SVG bytes live in `SvgStore`; it is
    ///   passed directly to `SvgInlineElement` / `paint_svg`.
    /// * `Err(msg)` — compilation failed; `msg` is shown as red text.
    ///
    /// Storing the result avoids re-compiling on every frame.  Typst
    /// compilation is synchronous and CPU-intensive, so doing it in `render`
    /// would stall the UI thread.
    cached: Result<(SharedString, f32, f32), String>,
}

impl LatexLabel {
    /// Create a new `LatexLabel` and immediately compile the equation.
    ///
    /// `compile` is called here (at construction time) so that `render` never
    /// has to block on Typst.  If compilation fails the error is stored in
    /// `cached` and displayed as red text at render time.
    pub fn new(latex: impl Into<SharedString>, font_size: f32, color: impl Into<Hsla>, svg_store: SvgStore) -> Self {
        let latex: SharedString = latex.into();
        // Compile eagerly so the first `render` call has a result ready.
        let cached = Self::compile(&latex, font_size, &svg_store);
        Self { latex, font_size, color: color.into(), svg_store, cached }
    }

    /// Compile a LaTeX string into an SVG and register it with the store.
    ///
    /// This is a **static** (associated) method rather than an instance method
    /// so it can be called from three different call sites without borrowing
    /// `self`:
    ///   * `new` — `self` doesn't exist yet.
    ///   * `set_latex` — borrows `self.svg_store` immutably while assigning to
    ///     `self.cached` (a mutable borrow of `self`).
    ///   * `set_font_size` — same pattern.
    ///
    /// Making it static lets all three sites share one implementation without
    /// lifetime conflicts.
    fn compile(
        latex: &str,
        font_size: f32,
        store: &SvgStore,
    ) -> Result<(SharedString, f32, f32), String> {
        // Runs the full LaTeX → Typst → SVG pipeline.
        let svg = latex_to_svg(latex, font_size)?;
        // Register the bytes in the shared store and get back the unique path key.
        // The key is a content-addressed or UUID string (see SvgStore::insert).
        let path = store.insert(svg.svg_bytes);
        Ok((path, svg.width_pt, svg.height_pt))
    }

    /// Replace the displayed equation with new LaTeX source.
    ///
    /// Skips recompilation if the new string equals the current one.
    /// After updating `cached`, calls `cx.notify()` to schedule a re-render.
    ///
    /// # Why `cx.notify()`?
    /// GPUI only re-renders a component when it is explicitly marked dirty.
    /// Without `cx.notify()`, the cached result would update in memory but the
    /// screen would not refresh until something else triggered a repaint.
    #[allow(dead_code)]
    pub fn set_latex(&mut self, latex: impl Into<SharedString>, cx: &mut Context<Self>) {
        let new_latex: SharedString = latex.into();
        if self.latex != new_latex {
            self.latex = new_latex;
            self.cached = Self::compile(&self.latex, self.font_size, &self.svg_store);
            // Mark this entity dirty so GPUI calls `render` on the next frame.
            cx.notify();
        }
    }

    /// Change the font size and recompile the equation at the new size.
    ///
    /// # Why `f32::EPSILON`?
    /// Floating-point arithmetic can produce values that differ by a tiny rounding
    /// error even when the "logical" value hasn't changed (e.g. `24.0 - 2.0 + 2.0`
    /// might not round-trip exactly to `24.0`).  Comparing with `f32::EPSILON`
    /// instead of `!=` prevents spurious recompilations and `cx.notify()` calls
    /// caused by floating-point noise.
    pub fn set_font_size(&mut self, size: f32, cx: &mut Context<Self>) {
        if (self.font_size - size).abs() > f32::EPSILON {
            self.font_size = size;
            self.cached = Self::compile(&self.latex, self.font_size, &self.svg_store);
            // Schedule a repaint now that the cached SVG has changed.
            cx.notify();
        }
    }
}

impl Render for LatexLabel {
    /// Paint the equation (or an error message) for the current frame.
    ///
    /// This method runs on every repaint, so it must be cheap.  All heavy work
    /// (Typst compilation) is pre-done in `compile`; here we just branch on the
    /// cached result and build the lightweight element tree.
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.cached {
            Ok((path, width_pt, height_pt)) => {
                // The SVG bytes live in SvgStore under `path`.  GPUI will call
                // `AssetSource::load(path)` during the paint phase, retrieve the
                // bytes, rasterise the SVG at the physical pixel resolution, and
                // tint every pixel with `self.color`.
                SvgInlineElement::new(
                    path.clone(),
                    px(*width_pt),  // logical width: pt == px at 72 dpi
                    px(*height_pt), // logical height: same reasoning
                    self.color,
                )
                .into_any()
            }
            Err(e) => {
                // Compilation failed — show a red error badge instead of crashing.
                // This lets the user see what went wrong without taking down the
                // whole window.
                div()
                    .child(format!("LaTeX Error: {}", e))
                    .text_color(rgb(0xFF0000))
                    .into_any()
            }
        }
    }
}
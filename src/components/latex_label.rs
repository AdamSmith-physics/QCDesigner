use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};
use gpui_kit::*;

use crate::components::SvgInlineElement;
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

    /// Pre-compiled render result.
    ///
    /// * `Ok((key, bytes, width_pt, height_pt))` — compilation succeeded.
    ///   `key` is a content-addressed path key used by GPUI's sprite atlas for
    ///   caching, and `bytes` are the raw SVG bytes passed directly to
    ///   `SvgInlineElement` / `paint_svg`.
    /// * `Err(msg)` — compilation failed; `msg` is shown as red text.
    ///
    /// Storing the result avoids re-compiling on every frame.  Typst
    /// compilation is synchronous and CPU-intensive, so doing it in `render`
    /// would stall the UI thread.
    cached: Result<(SharedString, Arc<[u8]>, f32, f32), String>,
}

impl LatexLabel {
    /// Create a new `LatexLabel` and immediately compile the equation.
    ///
    /// `compile` is called here (at construction time) so that `render` never
    /// has to block on Typst.  If compilation fails the error is stored in
    /// `cached` and displayed as red text at render time.
    pub fn new(latex: impl Into<SharedString>, font_size: f32, color: impl Into<Hsla>) -> Self {
        let latex: SharedString = latex.into();
        // Compile eagerly so the first `render` call has a result ready.
        let cached = Self::compile(&latex, font_size);
        Self { latex, font_size, color: color.into(), cached }
    }

    /// Compile a LaTeX string into an SVG and compute its content-hash cache key.
    fn compile(
        latex: &str,
        font_size: f32,
    ) -> Result<(SharedString, Arc<[u8]>, f32, f32), String> {
        // Runs the full LaTeX → Typst → SVG pipeline.
        let svg = latex_to_svg(latex, font_size)?;
        let mut hasher = DefaultHasher::new();
        svg.svg_bytes.hash(&mut hasher);
        let path: SharedString = format!("latex-svg/{:016x}.svg", hasher.finish()).into();
        let data: Arc<[u8]> = Arc::from(svg.svg_bytes);
        Ok((path, data, svg.width_pt, svg.height_pt))
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
            self.cached = Self::compile(&self.latex, self.font_size);
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
            self.cached = Self::compile(&self.latex, self.font_size);
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
            Ok((path, data, width_pt, height_pt)) => {
                SvgInlineElement::new(
                    path.clone(),
                    data.clone(),
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
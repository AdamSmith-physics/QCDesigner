// measured_width_element.rs
//
// A custom GPUI 0.2.2 [`Element`] that wraps any inner element (typically a
// `div()`), captures its resolved layout width during `prepaint`, calls a
// one-shot callback with that width, and then paints the inner element
// unchanged.
//
// ── Design rationale ────────────────────────────────────────────────────────
//
// The GPUI element lifecycle has three ordered phases per frame:
//
//   1. request_layout  — ask Taffy for a LayoutId; return per-element state
//                        that is threaded into the next phases.
//   2. prepaint        — receive the resolved Bounds<Pixels> from Taffy; do
//                        hitbox registration, capture measurements, etc.
//   3. paint           — issue GPU draw calls.
//
// Key insight: because we return the *inner* element's LayoutId from our own
// `request_layout`, Taffy assigns the inner element's computed size to *our*
// slot in the tree. The `bounds` that GPUI hands to our `prepaint` method is
// therefore exactly the inner element's layout bounds. We capture the width
// from there — no custom measure function needed.
//
// ── AnyElement threading ────────────────────────────────────────────────────
//
// `AnyElement` (the erased wrapper around any `Element`) must be driven
// through all three phases in order:
//
//   inner.request_layout(window, cx)  →  LayoutId
//   inner.prepaint(window, cx)        →  (internally fetches its own bounds)
//   inner.paint(window, cx)
//
// The canonical GPUI pattern for elements that wrap a child is to store the
// `AnyElement` in `RequestLayoutState` (returned from `request_layout`) so
// that GPUI threads a `&mut AnyElement` into `prepaint` and `paint`.
// `AnimationElement` in the GPUI source does exactly this.
//
// ── FnOnce callback ─────────────────────────────────────────────────────────
//
// Because `Element: 'static`, the callback must also be `'static`.
// We store it as `Option<Box<dyn FnOnce(...)>>` and `.take()` it in
// `prepaint`. Using `Option` lets us move out of `&mut self`; the `None`
// branch is unreachable in normal use (GPUI never calls prepaint twice).

use gpui::{AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId,
           IntoElement, LayoutId, Pixels, Window};

// ────────────────────────────────────────────────────────────────────────────
// Public constructor helper (mirrors `div()`, `anchored()`, etc.)
// ────────────────────────────────────────────────────────────────────────────

/// Wrap `inner` in a [`MeasuredWidthElement`].
///
/// `on_width` is invoked exactly once, during `prepaint`, with:
/// - the resolved `Pixels` width of the inner element,
/// - a `&mut Window` so you can schedule work (e.g. `cx.notify()`), and
/// - a `&mut App`.
///
/// After the callback, the inner element is prepainted and painted normally,
/// so it appears on screen exactly as it would without this wrapper.
///
/// # Example
///
/// ```LearningGPUI/src/bin/gpui-latex/measured_width_element.rs#L1-1
/// measured_width(
///     div().child("hello"),
///     |w, _window, _cx| println!("width = {w:?}"),
/// )
/// ```
pub fn measured_width(
    inner: impl IntoElement,
    on_width: impl FnOnce(Pixels, &mut Window, &mut App) + 'static,
) -> MeasuredWidthElement {
    MeasuredWidthElement {
        // Convert to AnyElement immediately so we can store it without
        // preserving the generic type parameter on the struct.
        inner: Some(inner.into_any_element()),
        on_width: Some(Box::new(on_width)),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Struct
// ────────────────────────────────────────────────────────────────────────────

/// A GPUI element that measures the resolved width of its child during
/// `prepaint` and fires a one-shot callback, then paints the child normally.
///
/// Construct via [`measured_width`].
pub struct MeasuredWidthElement {
    /// The wrapped child.  Stored as `Option` so we can `.take()` it in
    /// `request_layout` and move it into `RequestLayoutState`.
    inner: Option<AnyElement>,

    /// The one-shot callback.  Stored as `Option<Box<dyn FnOnce(…)>>` so we
    /// can `.take()` it inside `prepaint` (which only receives `&mut self`).
    ///
    /// `'static` is required because `Element: 'static`.  This means the
    /// closure cannot capture references with shorter lifetimes.  Use
    /// `Arc`/`Rc`-wrapped shared state if you need to communicate the
    /// measurement back to a view.
    on_width: Option<Box<dyn FnOnce(Pixels, &mut Window, &mut App) + 'static>>,
}

// ────────────────────────────────────────────────────────────────────────────
// IntoElement
// ────────────────────────────────────────────────────────────────────────────

/// `IntoElement` with `Element = Self` means the struct is its own element —
/// no extra wrapper or allocation.  This is the standard pattern used by
/// `Div`, `Anchored`, `AnimationElement`, and every other first-party GPUI
/// element that doesn't derive `RenderOnce`.
impl IntoElement for MeasuredWidthElement {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Element
// ────────────────────────────────────────────────────────────────────────────

impl Element for MeasuredWidthElement {
    // ── Associated types ────────────────────────────────────────────────────

    /// We thread the `AnyElement` through as request-layout state so that
    /// `prepaint` and `paint` receive a `&mut AnyElement` and can call
    /// `inner.prepaint(window, cx)` / `inner.paint(window, cx)` on it.
    ///
    /// This is the same pattern used by `AnimationElement` and `Component<C>`
    /// in the GPUI source.
    type RequestLayoutState = AnyElement;

    /// Nothing extra needed after prepaint; `()` suffices.
    type PrepaintState = ();

    // ── Identity ────────────────────────────────────────────────────────────

    /// No element ID — this element carries no cross-frame state.
    fn id(&self) -> Option<ElementId> {
        None
    }

    /// No source location — only needed for inspector / dev-tools.
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    // ── Phase 1: request_layout ─────────────────────────────────────────────
    //
    // Contract (from element.rs):
    //   fn request_layout(
    //       &mut self,
    //       id: Option<&GlobalElementId>,
    //       inspector_id: Option<&InspectorElementId>,
    //       window: &mut Window,
    //       cx: &mut App,
    //   ) -> (LayoutId, Self::RequestLayoutState);
    //
    // We delegate entirely to the inner element:
    //   • call inner.request_layout(window, cx) to get a LayoutId
    //   • return that LayoutId so *our* slot in the Taffy tree has the same
    //     dimensions as the inner element
    //   • return the AnyElement as RequestLayoutState so GPUI threads it
    //     into prepaint/paint for us

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        // Take ownership of the inner AnyElement (Option lets us move out of
        // &mut self).  Panics on a double-call, which GPUI never does.
        let mut inner = self
            .inner
            .take()
            .expect("MeasuredWidthElement::request_layout called more than once");

        // Register the inner element's layout with Taffy.
        // AnyElement::request_layout is public: fn(&mut self, &mut Window, &mut App) -> LayoutId
        let layout_id = inner.request_layout(window, cx);

        // Thread `inner` forward — GPUI will pass it back to us as
        // `request_layout: &mut AnyElement` in prepaint and paint.
        (layout_id, inner)
    }

    // ── Phase 2: prepaint ───────────────────────────────────────────────────
    //
    // Contract (from element.rs):
    //   fn prepaint(
    //       &mut self,
    //       id: Option<&GlobalElementId>,
    //       inspector_id: Option<&InspectorElementId>,
    //       bounds: Bounds<Pixels>,          // ← resolved layout bounds for OUR layout_id
    //       request_layout: &mut Self::RequestLayoutState,  // = &mut AnyElement
    //       window: &mut Window,
    //       cx: &mut App,
    //   ) -> Self::PrepaintState;
    //
    // Because we returned the inner element's LayoutId from request_layout,
    // `bounds` here is *that element's* resolved rect — exactly what we want.
    //
    // Order matters:
    //   1. Fire the callback with bounds.size.width   (measurement)
    //   2. Call inner.prepaint(window, cx)            (hitboxes, child layout)

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        inner: &mut Self::RequestLayoutState, // &mut AnyElement
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        // ── Step 1: fire the width callback ──────────────────────────────
        //
        // `.take()` moves out of the Option — required because FnOnce
        // consumes itself.  In normal use this is always Some; the None
        // branch is a no-op guard against hypothetical re-entrant calls.
        if let Some(on_width) = self.on_width.take() {
            on_width(bounds.size.width, window, cx);
        }

        // ── Step 2: prepaint the inner element ───────────────────────────
        //
        // AnyElement::prepaint is public:
        //   fn(&mut self, &mut Window, &mut App) -> Option<FocusHandle>
        //
        // Internally, Drawable::prepaint calls window.layout_bounds(layout_id)
        // to resolve its own bounds — it does NOT use the `bounds` we received
        // above.  The two values are identical (same LayoutId), so this is fine.
        //
        // We discard the returned Option<FocusHandle>; if you need focus
        // tracking, propagate it through PrepaintState instead.
        inner.prepaint(window, cx);
    }

    // ── Phase 3: paint ──────────────────────────────────────────────────────
    //
    // Contract (from element.rs):
    //   fn paint(
    //       &mut self,
    //       id: Option<&GlobalElementId>,
    //       inspector_id: Option<&InspectorElementId>,
    //       bounds: Bounds<Pixels>,
    //       request_layout: &mut Self::RequestLayoutState,  // = &mut AnyElement
    //       prepaint: &mut Self::PrepaintState,             // = &mut ()
    //       window: &mut Window,
    //       cx: &mut App,
    //   );

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        inner: &mut Self::RequestLayoutState, // &mut AnyElement
        _prepaint: &mut Self::PrepaintState,  // &mut ()
        window: &mut Window,
        cx: &mut App,
    ) {
        // AnyElement::paint is public: fn(&mut self, &mut Window, &mut App)
        inner.paint(window, cx);
    }
}

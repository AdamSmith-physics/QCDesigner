use gpui::*;

// --- MeasuredElement ---
//
// A transparent wrapper around any element that fires a one-shot callback
// during `prepaint` with the Taffy-resolved pixel size (width and height)
// of the inner element.
//
// • Delegates layout entirely to the inner element — same LayoutId, same
//   bounds — so it has no visual effect of its own.
// • `inner` and `on_measure` are stored as `Option` so they can be `.take()`n
//   in the respective single-call phases (request_layout / prepaint).
// • The callback receives the full `Size<Pixels>`, so callers can use just
//   the width, just the height, or both without changing the signature.
// • The callback should call `window.defer()` + `cx.notify()` so the state
//   update runs *after* the current draw phase, when GPUI will schedule a
//   new frame.

pub struct MeasuredElement {
    inner:      Option<AnyElement>,
    on_measure: Option<Box<dyn FnOnce(Size<Pixels>, &mut Window, &mut App) + 'static>>,
}

impl MeasuredElement {
    pub fn new(child: impl IntoElement) -> Self {
        Self { inner: Some(child.into_any_element()), on_measure: None }
     }

    pub fn on_measure(
        mut self,
        cb: impl FnOnce(Size<Pixels>, &mut Window, &mut App) + 'static,
     ) -> Self {
        self.on_measure = Some(Box::new(cb));
        self
    }
}

impl IntoElement for MeasuredElement {
    type Element = Self;
    fn into_element(self) -> Self { self }
}

impl Element for MeasuredElement {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> { None }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, AnyElement) {
        let mut inner = self.inner.take().expect("request_layout called twice");
        let layout_id = inner.request_layout(window, cx);
        (layout_id, inner)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        inner: &mut AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(cb) = self.on_measure.take() {
            cb(bounds.size, window, cx);
          }
        inner.prepaint(window, cx);
      }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        inner: &mut AnyElement,
        _prepaint: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        inner.paint(window, cx);
    }
}
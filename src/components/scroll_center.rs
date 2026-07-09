use gpui::*;
use gpui_component::scroll::{ScrollableElement, ScrollbarAxis};

/// Centres its child vertically when smaller than the available space; scrolls
/// when larger.
///
/// Wrap in a `flex_1().min_h(px(0.0))` container to fill remaining space in a
/// flex column without any explicit height calculations:
///
/// ```rust
/// div().flex_1().min_h(px(0.0))
///     .child(ScrollCenter::new(handle, content).p(px(10.0)))
/// ```
///
/// # Layout
/// ```
/// [outer div]          flex_1, relative, flex_col, on_scroll_wheel
///   [scroll area]      flex_1, min_h(0), overflow_scroll, track_scroll, flex_col
///     [content wrapper]  my_auto + mx_auto + padding
///       [child]
///   [scrollbar]        absolute overlay
/// ```
///
/// Centering uses CSS auto-margins: free space is absorbed equally on each side
/// to centre small content; margins collapse to zero on overflow so scrolling
/// works normally.
#[derive(IntoElement)]
pub struct ScrollCenter {
    scroll_handle: ScrollHandle,
    child: AnyElement,
    padding: Pixels,
    /// When `Some`, the inner content wrapper is given this as its minimum
    /// width with `flex_shrink_0()`.  This prevents the flex cross-axis
    /// `stretch` / shrink behaviour that would otherwise squeeze the wrapper
    /// to the viewport width when the window is narrower than the content,
    /// making horizontal scrolling impossible.
    min_content_width: Option<Pixels>,
}

impl ScrollCenter {
    pub fn new(scroll_handle: ScrollHandle, child: impl IntoElement) -> Self {
        Self {
            scroll_handle,
            child: child.into_any_element(),
            padding: px(0.0),
            min_content_width: None,
        }
    }

    pub fn p(mut self, padding: Pixels) -> Self {
        self.padding = padding;
        self
    }

    /// Set the minimum width of the content wrapper.
    /// Pass `self.content_width` (an `Option<Pixels>`) directly — `None` on
    /// the first frame is handled gracefully (no constraint applied).
    pub fn min_content_width(mut self, width: Option<Pixels>) -> Self {
        self.min_content_width = width;
        self
    }
}

impl RenderOnce for ScrollCenter {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
         // --- First-frame measurement pass ---
         // On the very first frame `min_content_width` is `None` because the
         // content has not been measured yet.  We render the child off-screen
         // (absolute, far outside the viewport) so:
         // • The element IS still in the GPUI render tree, so
         //   MeasuredElement's prepaint fires and captures the correct width.
         // • Nothing is visible to the user, so there is no flash of
         //   incorrectly-sized or un-centred content.
         // The deferred cx.notify() then schedules a second frame which renders
         // the content correctly from the start.
        // if self.min_content_width.is_none() {
        //     println!("Re-rendering");
        //     return div()
        //         .flex_1()
        //         .min_h(px(0.0))
        //         .relative()
        //         .child(
        //             div()
        //                 .absolute()
        //                 .left(px(-99999.))
        //                 .top(px(-99999.))
        //                 .child(self.child),
        //         );
        // }

         // --- Normal rendering (content has been measured) ---
        let scroll_handle = self.scroll_handle.clone();
        let line_height = window.line_height();

        div()
            .flex_1()
            .min_h(px(0.0))
            .relative()
            .flex()
            .flex_col()
            // Capture scroll events over the full div so gestures over empty
            // padding still scroll. stop_propagation prevents double-handling
            // by the inner overflow_scroll.
            .on_scroll_wheel(move |event: &ScrollWheelEvent, _window, cx| {
                let delta = event.delta.pixel_delta(line_height);
                let mut offset = scroll_handle.offset();
                offset.x += delta.x;
                offset.y += delta.y;
                scroll_handle.set_offset(offset);
                cx.stop_propagation();
            })
            .child(
                // flex_1 + min_h(0): fills remaining space and can shrink below
                // content height (default min-height:auto prevents shrinking).
                // overflow_scroll clips to the flex-computed layout bounds, so
                // the scroll viewport is always exactly the available space.
                div()
                    .id("scroll-center-area")
                    .flex_1()
                    .min_h(px(0.0))
                    .overflow_scroll()
                    .track_scroll(&self.scroll_handle)
                    .flex()
                    .flex_col()
                    .child({
                        // The wrapper is a flex centering container.
                        //
                        // align-self stays at the default (stretch) so it
                        // fills the viewport width when the viewport is wider
                        // than the content — justify_center then centres the
                        // content inside it.
                        //
                        // min_w(content + padding) + flex_shrink_0 ensure the
                        // wrapper never shrinks below the content width, so it
                        // overflows (rather than compresses) when the viewport
                        // is narrower, enabling horizontal scrolling.
                        //
                        // (The None branch below is now unreachable at runtime
                        // due to the early return above, but is kept for
                        // completeness.)
                        let wrapper = div().my_auto().p(self.padding);
                        let wrapper = if let Some(w) = self.min_content_width {
                            wrapper
                                .flex()
                                .justify_center()
                                .min_w(w + self.padding * 2.)
                                .flex_shrink_0()
                        } else {
                            wrapper.mx_auto()
                        };
                        wrapper.child(self.child)
                    }),
            )
            .scrollbar(&self.scroll_handle, ScrollbarAxis::Both)
    }
}
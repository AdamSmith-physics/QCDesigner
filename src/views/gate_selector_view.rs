use gpui::*;
use gpui_component::{
    tab::TabBar,
    v_flex
};

pub struct GateSelectorView {}

impl GateSelectorView {
    pub fn new(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| {
            Self {}
        })
    }
}

impl Render for GateSelectorView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                TabBar::new("underline")
                    .w_full()
                    .selected_index(0)
                    .child("Gate Selector")
            )
            .child("This is the gate selector!")
    }
}

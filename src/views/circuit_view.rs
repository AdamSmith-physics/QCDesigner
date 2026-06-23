use gpui::*;
use gpui_component::{v_flex};

pub struct CircuitView {}

impl CircuitView {
    pub fn new(_: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for CircuitView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child("Circuit View")
    }
}

use gpui::*;
use gpui_component::{
    button::Button,
    v_flex
};

pub struct CircuitView {}

impl CircuitView {
    pub fn new(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| {
            Self {}
        })
    }
}

impl Render for CircuitView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("This is the circuit view!")
            .child(
                Button::new("ok")
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

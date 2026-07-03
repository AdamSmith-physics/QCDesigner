use gpui::*;

use crate::utils::constants::gate_button as constants;

// --- gate_button ---
//
// Stateless builder for a selected button. It will display a number which
// shows the order in which it was selected. The caller supplies the click
// handler (built with `cx.listener(...)` at the call site), keeping this
// function free of any view-type dependency.
pub fn gate_button(
    button_size: f32,
    button_number: i32,
    on_click:    impl Fn(&MouseUpEvent, &mut Window, &mut App) + 'static,
) -> AnyElement {

    div()
        .group("gate-button")
        .flex()
        .justify_center()
        .w(px(button_size)).h(px(button_size))
        .rounded(px(4.0)).border(px(1.0))
        .border_color(constants::BUTTON_FG)
        .bg(constants::BUTTON_BG)
        .hover(|style| style.border(px(2.0)))
        .cursor_pointer()
        .child(format!("{}", button_number))
        .on_mouse_up(MouseButton::Left, on_click)
        .into_any_element()
}

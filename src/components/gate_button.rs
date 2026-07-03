use gpui::*;

// Colours used by gate_button — fixed regardless of theme.
const BUTTON_BG: Hsla = Hsla { h: 0., s: 0., l: 1.0,  a: 1.0 }; // white
const BUTTON_FG: Hsla = Hsla { h: 0., s: 0., l: 0.0,  a: 1.0 }; // black

// ── gate_button ───────────────────────────────────────────────────────────────
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
        .border_color(BUTTON_FG)
        .bg(BUTTON_BG)
        .hover(|style| style.border(px(2.0)))
        .cursor_pointer()
        .child(format!("{}", button_number))
        .on_mouse_up(MouseButton::Left, on_click)
        .into_any_element()
}

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::models::{RenderSettings, Gate};
use crate::utils::constants::gate_button as constants;
use crate::utils::dimensions;

// --- gate_button ---
//
// Stateless builder for a selected button. It will display a number which
// shows the order in which it was selected. The caller supplies the click
// handler (built with `cx.listener(...)` at the call site), keeping this
// function free of any view-type dependency.
pub fn gate_button(
    render_settings: RenderSettings,
    gate: Gate,
    is_selected: bool,
    on_click:    impl Fn(&MouseUpEvent, &mut Window, &mut App) + 'static,
) -> AnyElement {

    div()
        .group("gate-button")
        .p(dimensions::gate_button::LABEL_PADDING)
        .flex()
        .justify_center()
        .items_center()
        .min_w(px(render_settings.gate_size))
        .h(px(render_settings.gate_size))
        .rounded(px(render_settings.corner_radius))
        .border(px(render_settings.line_thickness))
        .border_color(constants::BUTTON_FG)
        .bg(constants::BUTTON_BG)
        .when(!is_selected, |this| {
            this
                .hover(|style| {
                    style
                        // .border(px(1.5*render_settings.line_thickness))
                        .shadow(vec![constants::BOX_SHADOW])
                })
        })
        .when(is_selected, |this| {
            this
                .shadow(vec![constants::BOX_BORDER])
                .border_color(constants::BUTTON_SELECTED)
        })
        .cursor_pointer()
        .child(gate.label.unwrap_or(format!("").into() ))
        .on_mouse_up(MouseButton::Left, on_click)
        .into_any_element()
}

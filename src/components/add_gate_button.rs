use gpui::*;

// Colours used by gate_button — fixed regardless of theme.
const BUTTON_BG: Hsla = Hsla { h: 0., s: 0., l: 1.0,  a: 0.95 }; // white
const BUTTON_FG: Hsla = Hsla { h: 0., s: 0., l: 0.65, a: 1. };   // mid-grey

const OPACITY_IDLE:  f32 = 0.05;
const OPACITY_HOVER: f32 = 1.0;

// Cross proportions relative to button_size.
const CROSS_ARM_FRAC:       f32 = 0.50; // total arm length  (50 % of button)
const CROSS_THICKNESS_FRAC: f32 = 0.05; // bar thickness     ( 5 % of button)

// ── gate_button ───────────────────────────────────────────────────────────────
//
// Stateless builder for an empty cell button.  The caller supplies the click
// handler (built with `cx.listener(...)` at the call site), keeping this
// function free of any view-type dependency.
//
// Visual layers:
//   1. Outer div  — white background + grey border; fades in on hover.
//   2. Cross canvas — centred "+"; invisible at idle, revealed on hover.
pub fn add_gate_button(
    button_size: f32,
    on_click:    impl Fn(&MouseUpEvent, &mut Window, &mut App) + 'static,
) -> AnyElement {
    let cross_canvas = canvas(
        |_bounds, _window, _cx| {},
        move |bounds, _state, window, _cx| {
            draw_cross(bounds, window);
        },
    )
    .absolute().top(px(0.0)).left(px(0.0)).right(px(0.0)).bottom(px(0.0));

    let cross_layer = div()
        .absolute().top(px(0.0)).left(px(0.0)).right(px(0.0)).bottom(px(0.0))
        .opacity(0.0)
        .group_hover("gate-button", |style| style.opacity(1.0))
        .child(cross_canvas);

    div()
        .group("gate-button")
        .relative()
        .w(px(button_size)).h(px(button_size))
        .rounded(px(4.0)).border(px(1.0))
        .border_color(BUTTON_FG)
        .bg(BUTTON_BG)
        .opacity(OPACITY_IDLE)
        .hover(|style| style.opacity(OPACITY_HOVER))
        .cursor_pointer()
        .child(cross_layer)
        .on_mouse_up(MouseButton::Left, on_click)
        .into_any_element()
}

// ── draw_cross ────────────────────────────────────────────────────────────────

fn draw_cross(bounds: Bounds<Pixels>, window: &mut Window) {
    let arm_half   = bounds.size.width * (CROSS_ARM_FRAC       * 0.5);
    let thick_half = bounds.size.width * (CROSS_THICKNESS_FRAC * 0.5);
    let cx = bounds.origin.x + bounds.size.width  * 0.5;
    let cy = bounds.origin.y + bounds.size.height * 0.5;

    window.paint_quad(fill(
        Bounds {
            origin: point(cx - arm_half,   cy - thick_half),
            size:   size(arm_half * 2.0, thick_half * 2.0),
        },
        BUTTON_FG,
    ));
    window.paint_quad(fill(
        Bounds {
            origin: point(cx - thick_half, cy - arm_half),
            size:   size(thick_half * 2.0, arm_half * 2.0),
        },
        BUTTON_FG,
    ));
}

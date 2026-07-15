use gpui::{Hsla, px, Point, BoxShadow};

pub mod titlebar {
    pub const TITLE_PLACEHOLDER: &str = "Circuit Title";
    pub const EXPORT_LABEL: &str = "Export";
}


pub mod root_view {
    pub const GATE_SETTINGS: &str = "Gate Settings";
    pub const CIRCUIT_SETTINGS: &str = "Circuit Settings";
}


pub mod gate_selector {
    pub const PANEL_TITLE: &str = "Gate Selector";
}


pub mod add_gate_button {
    use super::*;

    // Colours used by gate_button — fixed regardless of theme.
    pub const BUTTON_BG: Hsla = Hsla { h: 0., s: 0., l: 1.0,  a: 0.95 }; // white
    pub const BUTTON_FG: Hsla = Hsla { h: 0., s: 0., l: 0.65, a: 1. };   // mid-grey
    
    pub const OPACITY_IDLE:  f32 = 0.05;
    pub const OPACITY_HOVER: f32 = 1.0;
    
    // Cross proportions relative to button_size.
    pub const CROSS_ARM_FRAC:       f32 = 0.50; // total arm length  (50 % of button)
    pub const CROSS_THICKNESS_FRAC: f32 = 0.05; // bar thickness     ( 5 % of button)
}


pub mod gate_button {
    use super::*;

    // Colours used by gate_button — fixed regardless of theme.
    pub const BUTTON_BG: Hsla = Hsla { h: 0., s: 0., l: 1.0,  a: 1.0 }; // white
    pub const BUTTON_FG: Hsla = Hsla { h: 0., s: 0., l: 0.0,  a: 1.0 }; // black
    pub const BOX_SHADOW: BoxShadow = BoxShadow {
        color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 1.0 },
        offset: Point { x: px(0.0), y: px(1.0) },
        blur_radius: px(3.0),
        spread_radius: px(0.0)
    };
}
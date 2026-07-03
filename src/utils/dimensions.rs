use gpui::{Pixels, px};

// --- Global dimensions ---

pub const PADDING: Pixels = px(8.0);
pub const NUMBER_INPUT_WIDTH: Pixels = px(150.0);

// --- RootView panel sizes ---

pub mod root_view {
    use super::*;

    pub const RIGHT_PANEL_SIZE: Pixels = px(280.);
    pub const RIGHT_PANEL_MIN: Pixels = px(200.);  
    pub const RIGHT_PANEL_MAX: Pixels = px(400.);  

    pub const BOTTOM_PANEL_SIZE: Pixels = px(280.);
    pub const BOTTOM_PANEL_MIN: Pixels = px(200.);  
    pub const BOTTOM_PANEL_MAX: Pixels = px(400.);  
}
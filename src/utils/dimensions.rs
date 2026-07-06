use gpui::{Pixels, px};

// --- Global dimensions ---

pub const PADDING: Pixels = px(8.0);
pub const NUMBER_INPUT_WIDTH: Pixels = px(120.0);

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


pub mod render_settings {
    pub const GATE_SIZE_MAX: f32 = 100.0;
    pub const GATE_SIZE_MIN: f32 = 10.0;
    pub const GATE_SIZE_INCREMENT: f32 = 5.0;

    pub const LINE_THICKNESS_MAX: f32 = 5.0;
    pub const LINE_THICKNESS_MIN: f32 = 0.1;
    pub const LINE_THICKENSS_INCREMENT: f32 = 0.5;

    pub const CORNER_RADIUS_MAX: f32 = 50.0;
    pub const CORNER_RADIUS_MIN: f32 = 0.0;
    pub const CORNER_RADIUS_INCREMENT: f32 = 1.0;

    pub const ROW_GAP_MAX: f32 = 20.0;
    pub const ROW_GAP_MIN: f32 = 1.0;
    pub const ROW_GAP_INCREMENT: f32 = 1.0;

    pub const COLUMN_GAP_MAX: f32 = 20.0;
    pub const COLUMN_GAP_MIN: f32 = 1.0;
    pub const COLUMN_GAP_INCREMENT: f32 = 1.0;
}
use crate::utils::defaults::render_settings as defaults;

// --- End of imports ---


/// Settings for circuit-level rendering. 
pub struct RenderSettings {
    gate_size: f32,
    line_thickness: f32,
    row_padding: f32,
    column_padding: f32,
}

impl RenderSettings {
    pub fn new(gate_size: f32, line_thickness: f32, row_padding: f32, column_padding: f32) -> Self {
        Self {
            gate_size: gate_size,
            line_thickness: line_thickness,
            row_padding: row_padding,
            column_padding: column_padding,
        }
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self::new(
            defaults::GATE_SIZE,
            defaults::LINE_THICKNESS,
            defaults::ROW_PADDING,
            defaults::COLUMN_PADDING,
        )
    }
}
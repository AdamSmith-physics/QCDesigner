use crate::utils::defaults::render_settings as defaults;

// --- End of imports ---


/// Settings for circuit-level rendering. 
#[derive(Clone,Copy)]
pub struct RenderSettings {
    pub gate_size: f32,
    pub line_thickness: f32,
    pub corner_radius: f32,
    pub row_gap: f32,
    pub column_gap: f32,
}

impl RenderSettings {
    pub fn new(gate_size: f32, line_thickness: f32, corner_radius:f32, row_gap: f32, column_gap: f32) -> Self {
        Self {
            gate_size: gate_size,
            line_thickness: line_thickness,
            corner_radius: corner_radius,
            row_gap: row_gap,
            column_gap: column_gap,
        }
    }

    pub fn set_gate_size(&mut self, value: f32) {
        self.gate_size = value;
    }
    
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self::new(
            defaults::GATE_SIZE,
            defaults::LINE_THICKNESS,
            defaults::CORNER_RADIUS,
            defaults::ROW_GAP,
            defaults::COLUMN_GAP,
        )
    }
}
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

    
    // --- Gate size management ---
    
    pub fn set_gate_size(&mut self, value: f32) {
        self.gate_size = value;
    }

    pub fn increase_gate_size(&mut self) {
        const GATE_SIZE_MAX: f32 = 60.0;
        if self.gate_size + 0.5 < GATE_SIZE_MAX {
            self.gate_size = self.gate_size + 0.5;
        }
    }

    pub fn decrease_gate_size(&mut self) {
        const GATE_SIZE_MIN: f32 = 10.0;
        if self.gate_size - 0.5 > GATE_SIZE_MIN {
            self.gate_size = self.gate_size - 0.5;
        }
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
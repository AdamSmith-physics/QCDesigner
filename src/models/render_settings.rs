use crate::utils::defaults::render_settings as defaults;
use crate::utils::dimensions::render_settings as dimensions;

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
        if value <= dimensions::GATE_SIZE_MAX && value >= dimensions::GATE_SIZE_MIN {
            self.gate_size = value;
        }
    }

    pub fn increase_gate_size(&mut self) {
        if self.gate_size + dimensions::GATE_SIZE_INCREMENT <= dimensions::GATE_SIZE_MAX {
            self.gate_size = self.gate_size + dimensions::GATE_SIZE_INCREMENT;
        }
    }

    pub fn decrease_gate_size(&mut self) {
        if self.gate_size - dimensions::GATE_SIZE_INCREMENT >= dimensions::GATE_SIZE_MIN {
            self.gate_size = self.gate_size - dimensions::GATE_SIZE_INCREMENT;
        }
    }

    // --- Line thickeness management ---

    pub fn set_line_thickness(&mut self, value: f32) {
        if value <= dimensions::LINE_THICKNESS_MAX && value >= dimensions::LINE_THICKNESS_MIN {
            self.line_thickness = value;
        }
    }

    pub fn increase_line_thickness(&mut self) {
        if self.line_thickness + dimensions::LINE_THICKENSS_INCREMENT <= dimensions::LINE_THICKNESS_MAX {
            self.line_thickness = self.line_thickness + dimensions::LINE_THICKENSS_INCREMENT;
        }
    }

    pub fn decrease_line_thickness(&mut self) {
        if self.line_thickness - dimensions::LINE_THICKENSS_INCREMENT >= dimensions::LINE_THICKNESS_MIN {
            self.line_thickness = self.line_thickness - dimensions::LINE_THICKENSS_INCREMENT;
        }
    }

    // --- Corner radius management ---

    pub fn set_corner_radius(&mut self, value: f32) {
        if value <= dimensions::CORNER_RADIUS_MAX && value >= dimensions::CORNER_RADIUS_MIN {
            self.corner_radius = value;
        }
    }

    pub fn increase_corner_radius(&mut self) {
        if self.corner_radius + dimensions::CORNER_RADIUS_INCREMENT <= dimensions::CORNER_RADIUS_MAX {
            self.corner_radius = self.corner_radius + dimensions::CORNER_RADIUS_INCREMENT;
        }
    }

    pub fn decrease_corner_radius(&mut self) {
        if self.corner_radius - dimensions::CORNER_RADIUS_INCREMENT >= dimensions::CORNER_RADIUS_MIN {
            self.corner_radius = self.corner_radius - dimensions::CORNER_RADIUS_INCREMENT;
        }
    }

    // --- Row gap management ---

    pub fn set_row_gap(&mut self, value: f32) {
        if value <= dimensions::ROW_GAP_MAX && value >= dimensions::ROW_GAP_MIN {
            self.row_gap = value;
        }
    }

    pub fn increase_row_gap(&mut self) {
        if self.row_gap + dimensions::ROW_GAP_INCREMENT <= dimensions::ROW_GAP_MAX {
            self.row_gap = self.row_gap + dimensions::ROW_GAP_INCREMENT;
        }
    }

    pub fn decrease_row_gap(&mut self) {
        if self.row_gap - dimensions::ROW_GAP_INCREMENT >= dimensions::ROW_GAP_MIN {
            self.row_gap = self.row_gap - dimensions::ROW_GAP_INCREMENT;
        }
    }

    // --- Column gap management ---

    pub fn set_column_gap(&mut self, value: f32) {
        if value <= dimensions::COLUMN_GAP_MAX && value >= dimensions::COLUMN_GAP_MIN {
            self.column_gap = value;
        }
    }

    pub fn increase_column_gap(&mut self) {
        if self.column_gap + dimensions::COLUMN_GAP_INCREMENT <= dimensions::COLUMN_GAP_MAX {
            self.column_gap = self.column_gap + dimensions::COLUMN_GAP_INCREMENT;
        }
    }

    pub fn decrease_column_gap(&mut self) {
        if self.column_gap - dimensions::COLUMN_GAP_INCREMENT >= dimensions::COLUMN_GAP_MIN {
            self.column_gap = self.column_gap - dimensions::COLUMN_GAP_INCREMENT;
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
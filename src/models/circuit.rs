use gpui::Context;

use crate::models::{RenderSettings, Gate};
use crate::utils::{GateType, GateId, Coordinate};


// --- End of imports ---


/// The circuit model: grid dimensions, selected gates, and last-click tracking.
pub struct Circuit {
    pub rows:  usize,
    pub cols:  usize,
    
    pub render_settings: RenderSettings,
    gates: Vec<Gate>,
    pub selected_gate: Option<GateId>,
    pub last_clicked: Option<Coordinate>,
}

impl Circuit {
    /// Create a new circuit with the given grid dimensions.
    pub fn new(rows: usize, columns: usize) -> Self {
        let render_settings = RenderSettings::default();
        
        Self {
            rows: rows,
            cols: columns,
            render_settings: render_settings,
            gates: Vec::new(),
            selected_gate: None,
            last_clicked: None,
        }
    }
    
    /// Add a gate at the given coordinate.  No-op if already selected.
    pub fn add_gate(&mut self, coordinate: Coordinate, cx: &mut Context<Self>) {
        let new_gate = Gate::new(GateType::SingleQubit, coordinate, cx);
        self.gates.push(new_gate);

        self.last_clicked = Some(coordinate);
    } 
    
    /// Remove a gate at the given coordinate and clear last-clicked.
    pub fn remove_gate(&mut self, coordinate: &Coordinate) {
        for (ii, gate) in self.gates.clone().iter().enumerate() {
            if gate.coordinate() == *coordinate {
                self.gates.remove(ii);
            }
        }
        
        self.last_clicked = None;
    }

    /// Select a gate when clicked
    pub fn select_gate(&mut self, gate: Gate) {
        // Check it is a gate in the list
        if !self.gates.contains(&gate) {
            println!("Gate doesn't exist!");
        }

        self.selected_gate = Some(gate.id());
    }

    pub fn is_selected(&self, gate_id: GateId) -> bool {
        if let Some(selected_gate_id) = &self.selected_gate {
            if gate_id == *selected_gate_id {
                return true
            }
        }
        false
    }
    
    // /// Returns `true` if there is a gate at the given coordinate.
    // pub fn is_selected(&self, gate: &Gate) -> bool {
    //     if let Some(selected_gate) = &self.selected_gate {
    //         if selected_gate == gate {
    //             return true
    //         }
    //     }
    //     false
    // }

    pub fn deselect_gate(&mut self) {
        self.selected_gate = None;
    }

    pub fn selected_gate(&self) -> Option<&Gate> {
        if let Some(selected_id) = self.selected_gate {
            self.gates.iter().find(|&x| x.id() == selected_id)
        } else {
            None
        }
        
    }

    /// Like selected_gate, but returns a mutable reference so callers can
    /// modify the selected gate in place (e.g. updating its label).
    pub fn selected_gate_mut(&mut self) -> Option<&mut Gate> {
        if let Some(selected_id) = self.selected_gate {
            self.gates.iter_mut().find(|x| x.id() == selected_id)
        } else {
            None
        }
    }
    
    // /// Returns the index of the gate at `coordinate` (selection order), or 0 if not found.
    // pub fn get_gate_number(&self, coordinate: &Coordinate) -> i32 {
    //     let mut gate_number = 0;
    //     for (i, gate) in self.selected_gates.clone().iter().enumerate() {
    //         if gate == coordinate {
    //             gate_number = i;
    //         }
    //     }
    //     gate_number as i32
    // }

    pub fn get_gate_at_coordinate(&self, coordinate: &Coordinate) -> Option<GateId> {
        let gate = self.gates.iter().find(|&x| x.coordinate() == *coordinate);
        match gate {
            Some(gate) => Some(gate.id()),
            None => None,
        }     

        // self.gates.iter().find(|&x| x.coordinate() == *coordinate)
        // for gate in self.gates.clone().iter();
    }

    pub fn get_gate(&self, id: GateId) -> Option<&Gate> {
        self.gates.iter().find(|&x| x.id() == id)
    }

     // --- Row management ---

    pub fn set_rows(&mut self, num_rows: i64) {
        self.rows = num_rows as usize;
    }

    pub fn increase_rows(&mut self) {
        if self.rows < 10 {
            self.rows = self.rows + 1;
        }
    }
      
    pub fn decrease_rows(&mut self) {
        if self.rows > 1 {
            self.rows = self.rows - 1;
        }
    }

    
    
}

impl Default for Circuit {
    /// Creates a 10 × 10 circuit.  Use `Circuit::new(rows, cols)` for other sizes.
    fn default() -> Self { Self::new(3, 10) }
}
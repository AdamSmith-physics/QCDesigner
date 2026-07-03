/// Position in the circuit grid.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub column: usize
}

/// The circuit model: grid dimensions, selected gates, and last-click tracking.
pub struct Circuit {
    pub rows:  usize,
    pub cols:  usize,
    selected_gates: Vec<Coordinate>,
    pub last_clicked: Option<Coordinate>,
}

impl Circuit {
    /// Create a new circuit with the given grid dimensions.
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows: rows,
            cols: columns,
            selected_gates: Vec::new(),
            last_clicked: None,
        }
    }
    
    /// Add a gate at the given coordinate.  No-op if already selected.
    pub fn add_gate(&mut self, coordinate: Coordinate) {
        if self.is_selected(&coordinate) { return };
        self.selected_gates.push(coordinate);
        self.last_clicked = Some(coordinate);
    } 
    
    /// Remove a gate at the given coordinate and clear last-clicked.
    pub fn remove_gate(&mut self, coordinate: &Coordinate) {
        for (ii, gate) in self.selected_gates.clone().iter().enumerate(){
            if gate == coordinate {
                self.selected_gates.remove(ii);
            }
        }
        self.last_clicked = None;
    }
    
    /// Returns `true` if there is a gate at the given coordinate.
    pub fn is_selected(&self, coordinate: &Coordinate) -> bool {
        for gate in self.selected_gates.clone() {
            if gate == *coordinate {
                return true
            }
        }
        false
    }
    
    /// Returns the index of the gate at `coordinate` (selection order), or 0 if not found.
    pub fn get_gate_number(&self, coordinate: &Coordinate) -> i32 {
        let mut gate_number = 0;
        for (i, gate) in self.selected_gates.clone().iter().enumerate() {
            if gate == coordinate {
                gate_number = i;
            }
        }
        gate_number as i32
    }

     // --- Row management ---

    pub fn set_rows(&mut self, num_rows: i64) {
        self.rows = num_rows as usize;
     }

    pub fn decrease_rows(&mut self) {
        if self.rows > 1 {
            self.rows = self.rows - 1;
         }
     }

    pub fn increase_rows(&mut self) {
        if self.rows < 10 {
            self.rows = self.rows + 1;
         }
     }
    
}

impl Default for Circuit {
    /// Creates a 10 × 10 circuit.  Use `Circuit::new(rows, cols)` for other sizes.
    fn default() -> Self { Self::new(3, 10) }
}
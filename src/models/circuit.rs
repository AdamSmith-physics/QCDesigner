
#[derive(Clone,PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub column: usize
}


pub struct Circuit {
    pub rows:  usize,
    pub cols:  usize,
    selected_gates: Vec<Coordinate>,
    pub last_clicked: Option<(usize, usize)>,  // might move this out to an AppState?
}


impl Circuit {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows: rows,
            cols: columns,
            selected_gates: Vec::new(),
            last_clicked: None,
        }
    }
    
    pub fn add_gate(&mut self, coordinate: Coordinate) {
        if self.is_selected(&coordinate) { return };
        self.selected_gates.push(coordinate);
    } 
    
    pub fn remove_gate(&mut self, coordinate: &Coordinate) {
        for (ii, gate) in self.selected_gates.clone().iter().enumerate(){
            if gate == coordinate {
                self.selected_gates.remove(ii);
            }
        }
    }
    
    pub fn is_selected(&mut self, coordinate: &Coordinate) -> bool {
        for gate in self.selected_gates.clone() {
            if gate == *coordinate {
                return true
            }
        }
        false
    }
    
    pub fn get_gate_number(&self, coordinate: &Coordinate) -> i32 {
        let mut gate_number = 0;
        for (i, gate) in self.selected_gates.clone().iter().enumerate() {
            if gate == coordinate {
                gate_number = i;
            }
        }
        gate_number as i32
    }
}

impl Default for Circuit {
    /// Creates a 10 × 10 circuit.  Use `Circuit::new(rows, cols)` for other sizes.
    fn default() -> Self { Self::new(10, 10) }
}
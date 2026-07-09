use crate::utils::{GateId, Coordinate};

#[derive(Debug, Clone)]
pub struct Gate {
    id: GateId,
    slice: usize,
    qubits: Vec<usize>,
    label: Option<String>
    // future fields: gate type, label, etc.
}

impl Gate {
    pub fn new(coordinate: Coordinate) -> Self {
        Self {
            id: GateId::next(),
            slice: coordinate.column,
            qubits: vec![coordinate.row],
            label: None,
        }
    }

    pub fn id(&self) -> GateId {
        self.id
    }
}
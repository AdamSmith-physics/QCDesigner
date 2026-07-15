use gpui::Hsla;
use crate::utils::{GateId, GateType, Coordinate};

#[derive(Clone,PartialEq,Eq)]
pub struct Gate {
    id: GateId,
    pub gate_type: GateType,
    slice: usize,
    qubits: Vec<usize>,
    pub label: Option<String>
    // future fields
}

impl Gate {
    pub fn new(gate_type: GateType, coordinate: Coordinate) -> Self {
        Self {
            id: GateId::next(),
            gate_type: gate_type,
            slice: coordinate.column,
            qubits: vec![coordinate.row],
            label: Some(format!("Long String")),
        }
    }

    pub fn id(&self) -> GateId {
        self.id
    }

    pub fn coordinate(&self) -> Coordinate {
        Coordinate { 
            row: *self.qubits.iter().min().expect("Gate should have at least one qubit!"), 
            column: self.slice 
        }
    }
}
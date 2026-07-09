
#[derive(Clone)]
pub enum GateType {
    SingleQubit,
    TwoQubit,
    CNOT,
    Toffoli,
    MultiQubit,
    Measurement,
    Reset,
}
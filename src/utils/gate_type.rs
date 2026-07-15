
#[derive(Clone,PartialEq,Eq)]
pub enum GateType {
    SingleQubit,
    TwoQubit,
    CNOT,
    Toffoli,
    MultiQubit,
    Measurement,
    Reset,
}
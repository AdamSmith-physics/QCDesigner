use std::sync::atomic::{AtomicU64, Ordering};
use gpui_kit::ElementId;

static NEXT_GATE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GateId(u64);

impl GateId {
    pub fn next() -> Self {
        Self(NEXT_GATE_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<GateId> for ElementId {
    fn from(id: GateId) -> Self {
        ("gate", id.0).into()
    }
}

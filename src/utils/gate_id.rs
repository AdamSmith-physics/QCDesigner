use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_GATE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GateId(u64);

impl GateId {
    fn next() -> Self {
        Self(NEXT_GATE_ID.fetch_add(1, Ordering::Relaxed))
    }
}
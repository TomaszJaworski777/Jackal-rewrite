use std::sync::atomic::{Ordering, AtomicU64};

#[derive(Debug, Default)]
pub struct SearchStats {
    iterations: AtomicU64,
    cumulative_depth: AtomicU64,
}

impl SearchStats {
    pub fn iterations(&self) -> u64 {
        self.iterations.load(Ordering::Relaxed)
    }

    pub fn avg_depth(&self) -> u64 {
        self.cumulative_depth.load(Ordering::Relaxed) / self.iterations()
    }

    pub fn push_iteration(&self, depth: u16) {
        self.iterations.fetch_add(1, Ordering::Relaxed);
        self.cumulative_depth.fetch_add(depth as u64, Ordering::Relaxed);
    }
}
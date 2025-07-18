use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

pub struct SearchStats {
    iterations: AtomicU64,
    cumulative_depth: AtomicU64,
    timer: Instant,
}

impl SearchStats {
    pub fn new(_threads: usize) -> Self {
        SearchStats {
            iterations: AtomicU64::new(0),
            cumulative_depth: AtomicU64::new(0),
            timer: Instant::now(),
        }
    }

    pub fn iterations(&self) -> u64 {
        self.iterations.load(Ordering::Relaxed)
    }

    pub fn avg_depth(&self) -> u64 {
        self.cumulative_depth.load(Ordering::Relaxed) / self.iterations()
    }

    pub fn time_passesd(&self) -> u128 {
        self.timer.elapsed().as_millis()
    }

    pub fn push_iteration(&self, depth: u16) {
        self.iterations.fetch_add(1, Ordering::Relaxed);
        self.cumulative_depth
            .fetch_add(depth as u64, Ordering::Relaxed);
    }
}

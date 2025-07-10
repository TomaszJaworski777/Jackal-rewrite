use std::time::Duration;

use crate::SearchEngine;

const DEFAULT_BENCH_DEPTH: usize = 5;

impl SearchEngine {
    pub fn bench(&self, depth: Option<u8>) -> (u128, Duration) {
        (0, Duration::default())
    }
}
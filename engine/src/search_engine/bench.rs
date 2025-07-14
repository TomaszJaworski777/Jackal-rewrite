use std::time::Duration;

use crate::SearchEngine;

const DEFAULT_BENCH_DEPTH: u8 = 5;

impl SearchEngine {
    pub fn bench(&self, depth: Option<u8>) -> (u128, Duration) {
        let depth = depth.unwrap_or(DEFAULT_BENCH_DEPTH);
        (0, Duration::default())
    }
}
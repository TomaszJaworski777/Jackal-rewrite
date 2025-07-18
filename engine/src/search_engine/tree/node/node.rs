use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};

use chess::Move;

const SCORE_SCALE: u32 = 1024 * 64;

#[derive(Debug)]
pub struct Node {
    mv: AtomicU16,
    visit_count: AtomicU32,
    cumulative_score: AtomicU64,
    children_start_index: AtomicU32,
    children_count: AtomicU8,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            mv: AtomicU16::new(self.mv.load(Ordering::Relaxed)),
            visit_count: AtomicU32::new(self.visit_count.load(Ordering::Relaxed)),
            cumulative_score: AtomicU64::new(self.cumulative_score.load(Ordering::Relaxed)),
            children_start_index: AtomicU32::new(self.children_start_index.load(Ordering::Relaxed)),
            children_count: AtomicU8::new(self.children_count.load(Ordering::Relaxed)),
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            mv: AtomicU16::new(0),
            visit_count: AtomicU32::new(0),
            cumulative_score: AtomicU64::new(0),
            children_start_index: AtomicU32::new(0),
            children_count: AtomicU8::new(0),
        }
    }

    pub fn clear(&self, mv: Move) {
        self.mv.store(u16::from(mv), Ordering::Relaxed);
        self.visit_count.store(0, Ordering::Relaxed);
        self.cumulative_score.store(0, Ordering::Relaxed);
        self.children_start_index.store(0, Ordering::Relaxed);
        self.children_count.store(0, Ordering::Relaxed);
    }

    pub fn mv(&self) -> Move {
        Move::from(self.mv.load(Ordering::Relaxed))
    }

    pub fn visits(&self) -> u32 {
        self.visit_count.load(Ordering::Relaxed)
    }

    pub fn score(&self) -> f32 {
        let cumulative_score =
            self.cumulative_score.load(Ordering::Relaxed) as f64 / f64::from(SCORE_SCALE);
        (cumulative_score / f64::from(self.visits())) as f32
    }

    pub fn children_count(&self) -> u8 {
        self.children_count.load(Ordering::Relaxed)
    }

    pub fn add_visit(&self, score: f32) {
        self.visit_count.fetch_add(1, Ordering::Relaxed);
        self.cumulative_score.fetch_add(
            (score as f64 * f64::from(SCORE_SCALE)) as u64,
            Ordering::Relaxed,
        );
    }

    pub fn add_children(&self, start_index: usize, chilren_count: usize) {
        self.children_start_index
            .store(start_index as u32, Ordering::Relaxed);
        self.children_count
            .store(chilren_count as u8, Ordering::Relaxed);
    }

    pub fn map_children<F: FnMut(usize)>(&self, mut func: F) {
        for child_idx in 0..self.children_count() {
            func(self.children_start_index.load(Ordering::Relaxed) as usize + child_idx as usize)
        }
    }
}

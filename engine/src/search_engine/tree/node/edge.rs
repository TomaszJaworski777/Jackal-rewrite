use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};

use chess::Move;

use crate::{search_engine::tree::node::wdl_score::SCORE_SCALE, AtomicWDLScore, WDLScore};

#[derive(Debug)]
pub struct Edge {
    node_idx: AtomicUsize,
    mv: AtomicU16,
    visit_count: AtomicU32,
    cumulative_score: AtomicWDLScore,
    squared_score: AtomicU64,
    policy: AtomicU16,
}

impl Clone for Edge {
    fn clone(&self) -> Self {
        Self {
            node_idx: AtomicUsize::new(self.node_idx.load(Ordering::Relaxed)),
            mv: AtomicU16::new(self.mv.load(Ordering::Relaxed)),
            visit_count: AtomicU32::new(self.visit_count.load(Ordering::Relaxed)),
            cumulative_score: self.cumulative_score.clone(),
            squared_score: AtomicU64::new(self.squared_score.load(Ordering::Relaxed)),
            policy: AtomicU16::new(self.policy.load(Ordering::Relaxed)),
        }
    }
}

impl Edge {
    pub fn new(mv: Move) -> Self {
        Self {
            node_idx: AtomicUsize::new(usize::MAX),
            mv: AtomicU16::new(u16::from(mv)),
            visit_count: AtomicU32::new(0),
            cumulative_score: AtomicWDLScore::default(),
            squared_score: AtomicU64::new(0),
            policy: AtomicU16::new(0),
        }
    }

    #[inline]
    pub fn clear(&self, mv: Move) {
        self.mv.store(u16::from(mv), Ordering::Relaxed);
        self.visit_count.store(0, Ordering::Relaxed);
        self.cumulative_score.clear();
        self.squared_score.store(0, Ordering::Relaxed);
        self.policy.store(0, Ordering::Relaxed);
    }

    #[inline]
    pub fn node_index(&self) -> usize {
        self.node_idx.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn mv(&self) -> Move {
        Move::from(self.mv.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn visits(&self) -> u32 {
        self.visit_count.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn score(&self) -> WDLScore {
        self.cumulative_score.get_score_with_visits(self.visits())
    }

    #[inline]
    pub fn squared_score(&self) -> f64 {
        ((self.squared_score.load(Ordering::Relaxed) as f64) / f64::from(SCORE_SCALE)) / f64::from(self.visits())
    }

    #[inline]
    pub fn policy(&self) -> f64 {
        self.policy.load(Ordering::Relaxed) as f64 / f64::from(u16::MAX)
    }

    #[inline]
    pub fn set_node_index(&self, node_idx: usize) {
        self.node_idx.store(node_idx, Ordering::Relaxed)
    }

    #[inline]
    pub fn set_policy(&self, policy: f64) {
        self.policy.store((policy * f64::from(u16::MAX)) as u16, Ordering::Relaxed)
    }

    #[inline]
    pub fn add_visit(&self, score: WDLScore) {
        self.visit_count.fetch_add(1, Ordering::Relaxed);
        self.cumulative_score.add(score);
        
        let score = score.single(0.5) as f64;
        self.squared_score.fetch_add((score.powi(2) * f64::from(SCORE_SCALE)) as u64, Ordering::Relaxed);
    }
}

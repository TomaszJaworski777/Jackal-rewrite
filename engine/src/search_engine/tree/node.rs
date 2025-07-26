use std::{sync::atomic::{AtomicU16, AtomicU32, AtomicU8, Ordering}};

use chess::Move;

use crate::{networks::{AtomicWDLScore, WDLScore}, search_engine::tree::node::game_state::AtomicGameState};

mod game_state;

pub use game_state::GameState;

#[derive(Debug)]
pub struct Node {
    mv: AtomicU16,
    visit_count: AtomicU32,
    cumulative_score: AtomicWDLScore,
    children_start_index: AtomicU32,
    children_count: AtomicU8,
    state: AtomicGameState,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            mv: AtomicU16::new(self.mv.load(Ordering::Relaxed)),
            visit_count: AtomicU32::new(self.visit_count.load(Ordering::Relaxed)),
            cumulative_score: self.cumulative_score.clone(),
            children_start_index: AtomicU32::new(self.children_start_index.load(Ordering::Relaxed)),
            children_count: AtomicU8::new(self.children_count.load(Ordering::Relaxed)),
            state: self.state.clone(),
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            mv: AtomicU16::new(0),
            visit_count: AtomicU32::new(0),
            cumulative_score: AtomicWDLScore::default(),
            children_start_index: AtomicU32::new(0),
            children_count: AtomicU8::new(0),
            state: AtomicGameState::new(GameState::Ongoing)
        }
    }

    #[inline]
    pub fn clear(&self, mv: Move) {
        self.mv.store(u16::from(mv), Ordering::Relaxed);
        self.visit_count.store(0, Ordering::Relaxed);
        self.cumulative_score.clear();
        self.children_start_index.store(0, Ordering::Relaxed);
        self.children_count.store(0, Ordering::Relaxed);
        self.state.set(GameState::Ongoing);
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
    pub fn state(&self) -> GameState {
        self.state.get()
    }

    #[inline]
    pub fn set_state(&self, state: GameState) {
        self.state.set(state)
    }

    #[inline]
    pub fn score(&self) -> WDLScore {
        self.cumulative_score.get_score(self.visits())
    }

    #[inline]
    pub fn is_terminal(&self) -> bool {
        self.state() != GameState::Ongoing
    }

    #[inline]
    pub fn children_count(&self) -> u8 {
        self.children_count.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn add_visit(&self, score: WDLScore) {
        self.visit_count.fetch_add(1, Ordering::Relaxed);
        self.cumulative_score.add(score);
    }

    #[inline]
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

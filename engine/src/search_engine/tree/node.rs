use std::sync::{atomic::{AtomicU16, AtomicU32, Ordering}, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::search_engine::tree::node::game_state::AtomicGameState;

mod game_state;
mod wdl_score;
mod edge;

pub use game_state::GameState;
pub use wdl_score::{WDLScore, AtomicWDLScore};
pub use edge::Edge;

#[derive(Debug)]
pub struct Node {
    visit_count: AtomicU32,
    children: RwLock<Vec<Edge>>,
    state: AtomicGameState,
    threads: AtomicU16,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            visit_count: AtomicU32::new(self.visit_count.load(Ordering::Relaxed)),
            children: RwLock::new(self.children.read().unwrap().clone()),
            state: self.state.clone(),
            threads: AtomicU16::new(self.threads.load(Ordering::Relaxed)),
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            visit_count: AtomicU32::new(0),
            children: RwLock::new(Vec::new()),
            state: AtomicGameState::new(GameState::Ongoing),
            threads: AtomicU16::new(0),
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.visit_count.store(0, Ordering::Relaxed);
        self.children.write().unwrap().clear();
        self.state.set(GameState::Ongoing);
        self.threads.store(0, Ordering::Relaxed);
    }

    #[inline]
    pub fn visits(&self) -> u32 {
        self.visit_count.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn children(&self) -> RwLockReadGuard<Vec<Edge>> {
        self.children.read().unwrap()
    }

    #[inline]
    pub fn children_mut(&self) -> RwLockWriteGuard<Vec<Edge>> {
        self.children.write().unwrap()
    }

    #[inline]
    pub fn children_count(&self) -> usize {
        self.children().len()
    }

    #[inline]
    pub fn state(&self) -> GameState {
        self.state.get()
    }

    #[inline]
    pub fn threads(&self) -> u16 {
        self.threads.load(Ordering::Relaxed)
    }
    
    #[inline]
    pub fn is_terminal(&self) -> bool {
        self.state() != GameState::Ongoing
    }

    #[inline]
    pub fn set_state(&self, state: GameState) {
        self.state.set(state)
    }

    #[inline]
    pub fn inc_threads(&self, value: u16) -> u16 {
        self.threads.fetch_add(value, Ordering::Relaxed)
    }

    #[inline]
    pub fn dec_threads(&self, value: u16) -> u16 {
        self.threads.fetch_sub(value, Ordering::Relaxed)
    }

    #[inline]
    pub fn add_visit(&self) {
        self.visit_count.fetch_add(1, Ordering::Relaxed);
    }
}

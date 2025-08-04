use std::sync::{atomic::{AtomicU16, AtomicU32, AtomicU8, Ordering}, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

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
    policy: AtomicU16,
    threads: AtomicU8,
    lock: RwLock<bool>
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
            policy: AtomicU16::new(self.policy.load(Ordering::Relaxed)),
            threads: AtomicU8::new(self.threads.load(Ordering::Relaxed)),
            lock: RwLock::new(false)
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
            state: AtomicGameState::new(GameState::Ongoing),
            policy: AtomicU16::new(0),
            threads: AtomicU8::new(0),
            lock: RwLock::new(false),
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
        self.policy.store(0, Ordering::Relaxed);
        self.threads.store(0, Ordering::Relaxed);
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
    pub fn policy(&self) -> f64 {
        self.policy.load(Ordering::Relaxed) as f64 / f64::from(u16::MAX)
    }

    #[inline]
    pub fn threads(&self) -> u8 {
        self.threads.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn read_lock(&self) -> LockResult<RwLockReadGuard<'_, bool>> {
        self.lock.read()
    }

    #[inline]
    pub fn write_lock(&self) -> LockResult<RwLockWriteGuard<'_, bool>> {
        self.lock.write()
    }

    #[inline]
    pub fn set_state(&self, state: GameState) {
        self.state.set(state)
    }

    #[inline]
    pub fn set_policy(&self, policy: f64) {
        self.policy.store((policy * f64::from(u16::MAX)) as u16, Ordering::Relaxed)
    }

    #[inline]
    pub fn inc_threads(&self, value: u8) -> u8 {
        self.threads.fetch_add(value, Ordering::Relaxed)
    }

    #[inline]
    pub fn dec_threads(&self, value: u8) -> u8 {
        self.threads.fetch_sub(value, Ordering::Relaxed)
    }

    #[inline]
    pub fn score(&self) -> WDLScore {
        self.cumulative_score.get_score_with_visits(self.visits())
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
    pub fn add_visits(&self, count: u32, score: WDLScore) {
        self.visit_count.fetch_add(count, Ordering::Relaxed);
        self.cumulative_score.add(score * count);
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

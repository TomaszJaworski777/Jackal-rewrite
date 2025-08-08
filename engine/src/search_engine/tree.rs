use std::sync::{atomic::{AtomicUsize, Ordering}, LockResult, RwLockReadGuard, RwLockWriteGuard};

use chess::Move;

mod node;
mod tree_draw;
mod tree_utils;
mod pv_line;

pub use node::{Node, GameState, AtomicWDLScore, WDLScore};
pub use pv_line::PvLine;

use crate::search_engine::hash_table::HashTable;

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    idx: AtomicUsize,
    hash_table: HashTable,
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            idx: AtomicUsize::new(self.idx.load(Ordering::Relaxed)),
            hash_table: self.hash_table.clone()
        }
    }
}

impl Tree {
    pub fn from_bytes(megabytes: usize, hash_percentage: f64) -> Self {
        let bytes = megabytes * 1024 * 1024;
        let hash_bytes = (bytes as f64 * hash_percentage) as usize;
        let tree_size = (bytes - hash_bytes) / std::mem::size_of::<Node>();

        Self {
            nodes: vec![Node::new(); tree_size],
            idx: AtomicUsize::new(1),
            hash_table: HashTable::new(hash_bytes),
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.idx.store(1, Ordering::Relaxed);
        self.nodes[self.root_index()].clear(Move::NULL);
        self.hash_table.clear();
    }

    #[inline]
    pub fn current_index(&self) -> usize {
        self.idx.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn tree_size(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn hash_table(&self) -> &HashTable {
        &self.hash_table
    }


    #[inline]
    pub fn root_index(&self) -> usize {
        0
    }

    #[inline]
    pub fn get_node(&self, node_idx: usize) -> Node {
        self.nodes[node_idx].clone()
    }

    #[inline]
    pub fn get_root_node(&self) -> Node {
        self.nodes[self.root_index()].clone()
    }

    #[inline]
    pub fn add_visit(&self, node_idx: usize, score: WDLScore) {
        self.nodes[node_idx].add_visit(score)
    }

    #[inline]
    pub fn set_state(&self, node_idx: usize, state: GameState) {
        self.nodes[node_idx].set_state(state)
    }

    #[inline]
    pub fn inc_threads(&self, node_idx: usize, value: u8) -> u8 {
        self.nodes[node_idx].inc_threads(value)
    }

    #[inline]
    pub fn dec_threads(&self, node_idx: usize, value: u8) -> u8 {
        self.nodes[node_idx].dec_threads(value)
    }

    #[inline]
    pub fn read_lock(&self, node_idx: usize) -> LockResult<RwLockReadGuard<'_, bool>> {
        self.nodes[node_idx].read_lock()
    }

    #[inline]
    pub fn write_lock(&self, node_idx: usize) -> LockResult<RwLockWriteGuard<'_, bool>> {
        self.nodes[node_idx].write_lock()
    }
}

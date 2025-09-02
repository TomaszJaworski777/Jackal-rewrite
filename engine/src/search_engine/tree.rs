use std::sync::atomic::{AtomicUsize, Ordering};

use chess::Move;

mod node;
mod tree_draw;
mod tree_utils;
mod pv_line;

pub use node::{Node, GameState, AtomicWDLScore, WDLScore, Edge};
pub use pv_line::PvLine;

use crate::search_engine::{hash_table::HashTable};

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    root_edge: Edge,
    idx: AtomicUsize,
    hash_table: HashTable,
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            root_edge: self.root_edge.clone(),
            idx: AtomicUsize::new(self.idx.load(Ordering::Relaxed)),
            hash_table: self.hash_table.clone()
        }
    }
}

impl Tree {
    pub fn from_bytes(megabytes: usize, hash_percentage: f64) -> Self {
        let bytes = megabytes * 1024 * 1024;
        let hash_bytes = (bytes as f64 * hash_percentage) as usize;
        let tree_size = Self::bytes_to_size(bytes - hash_bytes);

        let root_edge = Edge::new(Move::NULL);
        root_edge.set_node_index(0);

        Self {
            nodes: vec![Node::new(); tree_size],
            root_edge,
            idx: AtomicUsize::new(1),
            hash_table: HashTable::new(hash_bytes),
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.idx.store(1, Ordering::Relaxed);
        self.root_edge.clear(Move::NULL);
        self.nodes[self.root_index()].clear();
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
    pub fn root_edge(&self) -> &Edge {
        &self.root_edge
    }

    #[inline]
    pub fn root_index(&self) -> usize {
        self.root_edge().node_index()
    }

    #[inline]
    pub fn get_root_node(&self) -> &Node {
        &self.nodes[self.root_index()]
    }

    #[inline]
    pub fn get_node(&self, node_idx: usize) -> &Node {
        &self.nodes[node_idx]
    }

    #[inline]
    pub fn get_child_copy(&self, node_idx: usize, child_idx: usize) -> Edge {
        self.nodes[node_idx].children()[child_idx].clone()
    }

    #[inline]
    pub fn create_node(&self, node_idx: usize, child_idx: usize, state: GameState) -> bool {
        let children = self.nodes[node_idx].children();
        let idx = self.idx.fetch_add(1, Ordering::Relaxed);

        if idx + 1 >= self.tree_size() {
            return false;
        }

        self.nodes[idx].clear();
        self.nodes[idx].set_state(state);   
        children[child_idx].set_node_index(idx);

        true
    }

    #[inline]
    pub fn add_visit(&self, node_idx: usize, child_idx: usize, score: WDLScore) {
        let children = self.get_node(node_idx).children();
        let edge = &children[child_idx];
        self.nodes[edge.node_index()].add_visit();
        edge.add_visit(score);
    }

    #[inline]
    pub fn add_root_visit(&self, score: WDLScore) {
        self.nodes[self.root_index()].add_visit();
        self.root_edge.add_visit(score);
    }

    #[inline]
    pub fn set_state(&self, node_idx: usize, state: GameState) {
        self.nodes[node_idx].set_state(state)
    }

    #[inline]
    pub fn inc_threads(&self, node_idx: usize, value: u16) -> u16 {
        self.nodes[node_idx].inc_threads(value)
    }

    #[inline]
    pub fn dec_threads(&self, node_idx: usize, value: u16) -> u16 {
        self.nodes[node_idx].dec_threads(value)
    }
}

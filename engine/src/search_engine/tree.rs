use std::ops::{Index, IndexMut};

use chess::Move;

mod node;
mod tree_draw;
mod tree_utils;
mod pv_line;

pub use node::{Node, GameState, AtomicWDLScore, WDLScore, NodeIndex};
pub use pv_line::PvLine;

use crate::search_engine::{hash_table::HashTable, tree::node::AtomicNodeIndex};

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    idx: AtomicNodeIndex,
    hash_table: HashTable,
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            idx: self.idx.clone(),
            hash_table: self.hash_table.clone()
        }
    }
}

impl Index<NodeIndex> for Tree {
    type Output = Node;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index.idx() as usize]
    }
}

impl IndexMut<NodeIndex> for Tree {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.nodes[index.idx() as usize]
    }
}

impl Tree {
    pub fn from_bytes(megabytes: usize, hash_percentage: f64) -> Self {
        let bytes = megabytes * 1024 * 1024;
        let hash_bytes = (bytes as f64 * hash_percentage) as usize;
        let tree_size = Self::bytes_to_size(bytes - hash_bytes);

        Self {
            nodes: vec![Node::new(); tree_size],
            idx: AtomicNodeIndex::new(NodeIndex::new(0, 1)),
            hash_table: HashTable::new(hash_bytes),
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.idx.store(NodeIndex::new(0, 1));
        self[self.root_index()].clear(Move::NULL);
        self.hash_table.clear();
    }

    #[inline]
    pub fn current_index(&self) -> NodeIndex {
        self.idx.load()
    }

    #[inline]
    pub fn max_size(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn hash_table(&self) -> &HashTable {
        &self.hash_table
    }


    #[inline]
    pub fn root_index(&self) -> NodeIndex {
        NodeIndex::new(0, 0)
    }
    
    #[inline]
    pub fn get_root_node(&self) -> &Node {
        &self[self.root_index()]
    }

    #[inline]
    pub fn add_visit(&self, node_idx: NodeIndex, score: WDLScore) {
        self[node_idx].add_visit(score)
    }

    #[inline]
    pub fn set_state(&self, node_idx: NodeIndex, state: GameState) {
        self[node_idx].set_state(state)
    }

    #[inline]
    pub fn inc_threads(&self, node_idx: NodeIndex, value: u8) -> u8 {
        self[node_idx].inc_threads(value)
    }

    #[inline]
    pub fn dec_threads(&self, node_idx: NodeIndex, value: u8) -> u8 {
        self[node_idx].dec_threads(value)
    }
}

use chess::Move;

mod node;
mod tree_draw;
mod tree_utils;
mod pv_line;
mod tree_content;

pub use node::{Node, GameState, AtomicWDLScore, WDLScore, Edge, NodeIndex};
pub use pv_line::PvLine;
pub use tree_content::TreeContent;

use crate::search_engine::{hash_table::HashTable};

#[derive(Debug)]
pub struct Tree {
    content: TreeContent,
    root_edge: Edge,
    hash_table: HashTable,
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            root_edge: self.root_edge.clone(),
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
        root_edge.set_node_index(NodeIndex::from(0));

        Self {
            content: TreeContent::with_capacity(tree_size),
            root_edge,
            hash_table: HashTable::new(hash_bytes),
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.content.clear();
        self.content[self.root_index()].clear();
        self.hash_table.clear();

        self.root_edge.clear(Move::NULL);
        self.root_edge.set_node_index(NodeIndex::from(0));
    }

    #[inline]
    pub fn current_index(&self) -> usize {
        self.content.current_size()
    }

    #[inline]
    pub fn tree_size(&self) -> usize {
        self.content.max_size()
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
    pub fn root_index(&self) -> NodeIndex {
        self.root_edge().node_index()
    }

    #[inline]
    pub fn get_root_node(&self) -> &Node {
        &self.content[self.root_index()]
    }

    #[inline]
    pub fn get_node(&self, node_idx: NodeIndex) -> &Node {
        &self.content[node_idx]
    }

    #[inline]
    pub fn get_child_clone(&self, node_idx: NodeIndex, child_idx: usize) -> Edge {
        self.content[node_idx].children()[child_idx].clone()
    }

    #[inline]
    pub fn create_node(&self, node_idx: NodeIndex, child_idx: usize, state: GameState) -> Option<NodeIndex> {
        let children = self.content[node_idx].children_mut();

        let node_idx = children[child_idx].node_index();
        if !node_idx.is_null() {
            return Some(node_idx);
        } 

        let node_idx = self.content.reserve_node()?;

        self.content[node_idx].clear();
        self.content[node_idx].set_state(state);   
        children[child_idx].set_node_index(node_idx);

        Some(node_idx)
    }

    #[inline]
    pub fn add_visit(&self, node_idx: NodeIndex, child_idx: usize, score: WDLScore) {
        let children = self.get_node(node_idx).children();
        let edge = &children[child_idx];
        self.content[edge.node_index()].add_visit();
        edge.add_visit(score);
    }

    #[inline]
    pub fn add_root_visit(&self, score: WDLScore) {
        self.content[self.root_index()].add_visit();
        self.root_edge.add_visit(score);
    }

    #[inline]
    pub fn set_state(&self, node_idx: NodeIndex, state: GameState) {
        self.content[node_idx].set_state(state)
    }

    #[inline]
    pub fn inc_threads(&self, node_idx: NodeIndex, value: u16) -> u16 {
        self.content[node_idx].inc_threads(value)
    }

    #[inline]
    pub fn dec_threads(&self, node_idx: NodeIndex, value: u16) -> u16 {
        self.content[node_idx].dec_threads(value)
    }
}

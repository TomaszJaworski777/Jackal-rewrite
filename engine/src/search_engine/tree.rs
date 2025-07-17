use std::sync::atomic::{AtomicUsize, Ordering};

use chess::{ChessBoard, Move};

use crate::search_engine::tree::node::Node;

mod node;
mod tree_utils;
mod tree_draw;

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    idx: AtomicUsize
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self { 
            nodes: self.nodes.clone(), 
            idx: AtomicUsize::new(self.idx.load(Ordering::Relaxed)) 
        }
    }
}

impl Tree {
    pub fn from_bytes(bytes: usize) -> Self {
        let tree_size = bytes / std::mem::size_of::<Node>();
        println!("tree size: {tree_size}");

        let tree = Self { 
            nodes: vec![Node::new(); tree_size], 
            idx: AtomicUsize::new(1)
        };

        tree.reset_root();

        tree
    }

    pub fn clear(&self) {
        self.idx.store(1, Ordering::Relaxed);
        self.reset_root();
    }

    pub fn reset_root(&self) {
        self.nodes[0].clear(Move::NULL);
    }

    pub fn root_node(&self) -> &Node {
        &self.nodes[0]
    }

    pub fn get_node(&self, node_idx: usize) -> &Node {
        &self.nodes[node_idx]
    }

    pub fn expand_node(&self, node_idx: usize, board: &ChessBoard) -> bool {
        assert_eq!(self.nodes[node_idx].children_count(), 0, "{node_idx}");

        let mut moves = Vec::new();
        board.map_legal_moves(|mv| moves.push(mv));
        
        let start_index = self.reserve_nodes(moves.len());

        if start_index + moves.len() >= self.nodes.len() {
            return false;
        }

        self.nodes[node_idx].add_children(start_index, moves.len());

        for (idx, mv) in moves.into_iter().enumerate() {
            self.nodes[start_index + idx].clear(mv);
        }

        true
    }

    pub fn select_child<F: FnMut(&Node) -> f64>(&self, parent_idx: usize, mut key: F) -> Option<usize> {
        let mut best_idx = None;
        let mut best_score = f64::NEG_INFINITY;

        self.nodes[parent_idx].map_children(|child_idx| {
            let new_score = key(&self.nodes[child_idx]);
            if new_score > best_score {
                best_idx = Some(child_idx);
                best_score = new_score;
            }
        });

        best_idx
    }

    fn reserve_nodes(&self, count: usize) -> usize {
        self.idx.fetch_add(count, Ordering::Relaxed)
    }
}
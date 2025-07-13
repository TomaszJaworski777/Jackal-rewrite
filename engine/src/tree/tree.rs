use std::sync::atomic::{AtomicUsize, Ordering};

use chess::{ChessBoard, Move};

use crate::tree::node::Node;

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

        let tree = Self { 
            nodes: vec![Node::new(); tree_size], 
            idx: AtomicUsize::new(1)
        };

        tree.reset_root();

        tree
    }

    pub fn clear(&self) {
        self.idx.store(0, Ordering::Relaxed);
        self.reset_root();
    }

    pub fn reset_root(&self) {
        self.nodes[0].clear(Move::NULL);
    }

    pub fn root_node(&self) -> &Node {
        &self.nodes[0]
    }

    pub fn expand_node(&self, node_idx: usize, board: &ChessBoard) {
        let mut moves = Vec::new();
        board.map_legal_moves(|mv| moves.push(mv));
        
        let start_index = self.reserve_nodes(moves.len());

        self.nodes[node_idx].add_children(start_index, moves.len());

        for (idx, mv) in moves.into_iter().enumerate() {
            self.nodes[start_index + idx].clear(mv);
        }
    }

    fn reserve_nodes(&self, count: usize) -> usize {
        self.idx.fetch_add(count, Ordering::Relaxed)
    }
}
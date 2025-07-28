use std::sync::atomic::{AtomicUsize, Ordering};

use chess::{ChessBoard, Move};

mod node;
mod tree_draw;
mod tree_utils;
mod pv_line;

pub use node::{Node, GameState};

use crate::networks::{PolicyNetwork, WDLScore};

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    idx: AtomicUsize,
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            idx: AtomicUsize::new(self.idx.load(Ordering::Relaxed)),
        }
    }
}

impl Tree {
    pub fn from_bytes(megabytes: usize) -> Self {
        let bytes = megabytes * 1024 * 1024;
        let tree_size = bytes / std::mem::size_of::<Node>();

        Self {
            nodes: vec![Node::new(); tree_size],
            idx: AtomicUsize::new(1),
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.idx.store(1, Ordering::Relaxed);
        self.nodes[self.root_index()].clear(Move::NULL)
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

    pub fn expand_node(&self, node_idx: usize, board: &ChessBoard) -> bool {
        assert_eq!(
            self.nodes[node_idx].children_count(),
            0,
            "Node {node_idx} already have children."
        );

        let policy_inputs = PolicyNetwork.get_inputs(board);

        let mut moves = Vec::new();
        let mut policy = Vec::with_capacity(board.occupancy().pop_count() as usize);
        let mut max = 0f32;
        let mut total = 0f32;

        board.map_legal_moves(|mv| {
            moves.push(mv);
            let p = PolicyNetwork.forward(board, &policy_inputs, mv);
            policy.push(p);
            max = max.max(p);
        });

        let start_index = self.reserve_nodes(moves.len());

        if start_index + moves.len() >= self.nodes.len() {
            return false;
        }

        for p in policy.iter_mut() {
            *p = (*p - max).exp();
            total += *p;
        }

        self.nodes[node_idx].add_children(start_index, moves.len());

        for (idx, mv) in moves.into_iter().enumerate() {
            let p = if policy.len() == 1 {
                1.0
            } else {
                policy[idx] / total
            };

            self.nodes[start_index + idx].clear(mv);
            self.nodes[start_index + idx].set_policy(p as f64);
        }

        true
    }

    #[inline]
    fn reserve_nodes(&self, count: usize) -> usize {
        self.idx.fetch_add(count, Ordering::Relaxed)
    }
}

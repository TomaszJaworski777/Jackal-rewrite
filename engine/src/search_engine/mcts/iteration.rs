use chess::{ChessPosition, ZobristKey};

use crate::SearchEngine;

mod select_expand;
mod simulate;
mod backpropagate;

impl SearchEngine {
    pub(super) fn perform_iteration(
        &self,
        position: &mut ChessPosition,
        depth: &mut u64,
        castle_mask: &[u8; 64],
    ) -> bool { 
        let mut selection_stack: Vec<(usize, ZobristKey)> = Vec::new();

        let selected_node = self.select_and_expand(position, &mut selection_stack, castle_mask);

        *depth = (selection_stack.len() - 1).max(0) as u64;

        if selected_node.is_none() {
            return false;
        }

        let node_idx = selected_node.unwrap();

        let score = self.simulate(node_idx, position).reversed();

        self.backpropagate(&selection_stack, score);

        true
    }
}
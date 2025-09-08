use chess::ChessPosition;

use crate::{search_engine::tree::NodeIndex, SearchEngine, WDLScore};

mod select;
mod simulate;
mod backpropagate;

impl SearchEngine {
    pub(super) fn perform_iteration<const ROOT: bool>(
        &self,
        node_idx: NodeIndex,
        position: &mut ChessPosition,
        depth: &mut f64,
        castle_mask: &[u8; 64],
    ) -> Option<WDLScore> { 
        let hash = position.board().hash();
        let node: crate::Node = self.tree().get_node(node_idx);
        let score = if !ROOT && (node.is_terminal() || node.visits() == 0) {
            self.simulate(node_idx, position)
        } else {
            *depth += 1.0;

            if node.children_count() == 0 {
                if !self.tree().expand_node(node_idx, *depth, position.board(), self.options()) {
                    return None;
                }
            }

            let new_index = self.select(node_idx, *depth);

            position.make_move(self.tree().get_node(new_index).mv(), castle_mask);

            self.tree().inc_threads(new_index, 1);

            let score = self.perform_iteration::<false>(new_index, position, depth, castle_mask);

            self.tree().dec_threads(new_index, 1);

            score?
        }.reversed();

        self.backpropagate(node_idx, score, hash);

        Some(score)
    }
}
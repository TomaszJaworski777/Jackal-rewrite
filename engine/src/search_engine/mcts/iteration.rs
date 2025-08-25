use chess::ChessPosition;

use crate::{search_engine::Edge, SearchEngine, WDLScore};

mod select;
mod simulate;
mod backpropagate;

impl SearchEngine {
    pub(super) fn perform_iteration<const ROOT: bool>(
        &self,
        node_idx: usize,
        parent_edge: &Edge,
        position: &mut ChessPosition,
        depth: &mut f64,
        castle_mask: &[u8; 64],
    ) -> Option<WDLScore> { 
        let parent_hash = position.board().hash();
        let mut child_hash = None;

        let node = self.tree().get_node_copy(node_idx);
        let score = if !ROOT && (node.is_terminal() || node.visits() == 0) { //TODO: Test condition where edge.visits() is 0
            self.simulate(node_idx, position)
        } else {
            *depth += 1.0;

            self.tree().expand_node(node_idx, parent_edge, *depth, position.board(), self.options());

            let child_idx = self.select(node_idx, parent_edge, *depth);

            let mut edge = self.tree().get_child_copy(node_idx, child_idx);

            position.make_move(edge.mv(), castle_mask);
            child_hash = Some(position.board().hash());

            if edge.node_index() == usize::MAX {
                if !self.tree().create_node(node_idx, child_idx) {
                    return None;
                }

                edge = self.tree().get_child_copy(node_idx, child_idx);
            }
            
            let new_node = edge.node_index();
            
            self.tree().inc_threads(child_idx, 1);
            
            let score = self.perform_iteration::<false>(new_node, &edge, position, depth, castle_mask);

            self.tree().dec_threads(child_idx, 1);

            let score = score?;

            self.backpropagate(node_idx, child_idx, score);

            score
        };

        //Totally not a monty yoink
        if let Some(hash) = child_hash {
            self.tree().hash_table().push(hash, score.reversed());
        } else {
            self.tree().hash_table().push(parent_hash, score);
        }

        Some(score.reversed())
    }
}
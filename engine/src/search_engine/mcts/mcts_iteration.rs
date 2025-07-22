use chess::{ChessPosition, Piece, Side};

use crate::{search_engine::tree::Tree, SearchEngine};

impl SearchEngine {
    pub(super) fn perform_iteration(
        &self,
        tree: &Tree,
        node_idx: usize,
        position: &mut ChessPosition,
        depth: &mut u64,
        castle_mask: &[u8; 64],
    ) -> Option<f32> {
        let score = {
            if tree.get_node(node_idx).children_count() == 0 {
                if !tree.expand_node(node_idx, position.board()) {
                    return None;
                }

                Some(get_position_score(position, self.current_position()))
            } else {
                let new_index = tree.select_child_by_key(node_idx, |node| {
                    let score = if node.visits() == 0 {
                        0.5
                    } else {
                        node.score() as f64
                    };
                    ucb1(score, 2.0, tree.get_node(node_idx).visits(), node.visits())
                });

                assert_ne!(new_index, None);

                let new_index = new_index.unwrap();

                position.make_move(tree.get_node(new_index).mv(), castle_mask);

                *depth += 1;
                self.perform_iteration(tree, new_index, position, depth, castle_mask)
            }
        };

        if score.is_none() {
            return None;
        }

        let score = score.unwrap();
        tree.add_visit(node_idx, score);

        Some(1.0 - score)
    }
}

fn ucb1(score: f64, c: f64, parent_visits: u32, child_visits: u32) -> f64 {
    score + c * (f64::from(parent_visits.max(1)).ln() / f64::from(child_visits.max(1)))
}

fn get_position_score(position: &ChessPosition, root_position: &ChessPosition) -> f32 {
    let result = if is_draw(position, root_position) {
        0
    } else {
        calculate_material(position)
    };

    return sigmoid(result);
}

fn sigmoid(x: i32) -> f32 {
    1.0 / (1.0 + f32::exp(-x as f32 / 450.0))
}

fn is_draw(position: &ChessPosition, root_position: &ChessPosition) -> bool {
    if position.board().half_moves() >= 100 {
        return true;
    }

    let key = position.board().hash();
    let history_repetitions = position.history().get_repetitions(key);
    let search_repetitions = root_position.history().get_repetitions(key) - history_repetitions;

    if history_repetitions >=3 || search_repetitions >=2 || history_repetitions + search_repetitions >= 3 {
        return true;
    }

    false
}

fn calculate_material(position: &ChessPosition) -> i32 {
    let mut result = 0;
    
    const PIECE_VALUES: [i32; 6] = [100, 300, 330, 500, 1000, 0];

    for side in [Side::WHITE, Side::BLACK] {
        for piece_idx in u8::from(Piece::PAWN)..=u8::from(Piece::KING) {
            let piece = Piece::from(piece_idx);
            let piece_count = position.board().piece_mask_for_side(piece, side).pop_count();
            result += piece_count as i32 * PIECE_VALUES[piece_idx as usize];
        }

        result = -result;
    }

    result * if position.board().side() == Side::WHITE { 1 } else { -1 }
}
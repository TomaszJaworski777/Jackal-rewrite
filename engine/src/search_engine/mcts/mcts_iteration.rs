use chess::ChessPosition;

use crate::search_engine::tree::Tree;

pub fn perform_iteration(
    tree: &Tree,
    node_idx: usize,
    position: &mut ChessPosition,
    depth: &mut u16,
    castle_mask: &[u8; 64],
) -> Option<f32> {
    let score = {
        if tree.get_node(node_idx).children_count() == 0 {
            if !tree.expand_node(node_idx, position.board()) {
                return None;
            }

            Some(0.0)
        } else {
            let new_index = tree.select_child(node_idx, |node| {
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
            perform_iteration(tree, new_index, position, depth, castle_mask)
        }
    };

    if score.is_none() {
        return None;
    }

    let score = score.unwrap();
    tree.add_visit(node_idx, score);

    Some(1.0 - score)
}

fn ucb1(score: f64, c: f64, parent_visits: u32, child_visits: u32) -> f64 {
    score + c * (f64::from(parent_visits.max(1)).ln() / f64::from(child_visits.max(1)))
}

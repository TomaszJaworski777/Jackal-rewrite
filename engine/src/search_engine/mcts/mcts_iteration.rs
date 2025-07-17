use chess::ChessPosition;

use crate::search_engine::tree::Tree;

pub fn perform_iteration(tree: &Tree, position: &mut ChessPosition, depth: &mut u16) -> bool {
    let mut current_node_idx = 0;
    let mut iteration_history = Vec::new();
    iteration_history.push(current_node_idx);

    //1. Exploration
    loop {
        let new_index = tree.select_child(current_node_idx, |node| {
            let score = if node.visits() == 0 { 0.5 } else { node.score() as f64 };
        
            ucb1(score, 2.0, tree.get_node(current_node_idx).visits(), node.visits())
        } );

        if let Some(new_index) = new_index {
            current_node_idx = new_index;
            position.make_move(tree.get_node(current_node_idx).mv());
            iteration_history.push(current_node_idx);
            *depth += 1;
        } else {
            break;
        }
    }

    //2. Expansion
    if !tree.expand_node(current_node_idx, position.board()) {
        return false;
    }

    //3. Simulation
    let score = 0.5;

    //4. Backpropagation
    let mut alternate_score = false;
    for &node_idx in iteration_history.iter().rev() {
        tree.get_node(node_idx).add_visist(if alternate_score { 1.0 - score } else { score });
        alternate_score = !alternate_score;
    }

    true
}

fn ucb1(score: f64, c: f64, parent_visits: u32, child_visits: u32) -> f64 {
    score + c * (f64::from(parent_visits.max(1)).ln() / f64::from(child_visits.max(1)))
}
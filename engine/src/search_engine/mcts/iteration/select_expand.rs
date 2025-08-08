use chess::{ChessPosition, ZobristKey};

use crate::{search_engine::engine_options::EngineOptions, Node, SearchEngine, WDLScore};

impl SearchEngine {
    pub(super) fn select_and_expand(&self, position: &mut ChessPosition, selection_stack: &mut Vec<(usize, ZobristKey)>, castle_mask: &[u8; 64]) -> Option<usize> {
        let mut node_idx = self.tree().root_index();

        selection_stack.push((node_idx, position.board().hash()));

        loop {
            let parent_node = self.tree().get_node(node_idx);

            let cpuct = get_cpuct(&self.options(), &parent_node);

            node_idx = self.tree().select_child_by_key(node_idx, |child_node| {
                let score = get_score(&parent_node.score(), child_node, child_node.visits()).single(0.5) as f64;
                let exploration_factor = f64::from(parent_node.visits().max(1)).sqrt() / f64::from(child_node.visits() + 1);
                score + cpuct * child_node.policy() * exploration_factor
            }).expect("Failed to select a valid node.");

            position.make_move(self.tree().get_node(node_idx).mv(), castle_mask);

            selection_stack.push((node_idx, position.board().hash()));
            
            self.tree().inc_threads(node_idx, 1);
            
            let node = self.tree().get_node(node_idx);

            if node.visits() == 0 || node.is_terminal() {
                break;
            }

            if self.tree().get_node(node_idx).children_count() == 0 {
                if !self.tree().expand_node(node_idx, position.board(), self.options()) {
                    return None;
                }
            }
        }

        Some(node_idx)
    }
}

fn get_score(parent_score: &WDLScore, child_node: &Node, child_visits: u32) -> WDLScore {
    let mut score = if child_visits == 0 {
        parent_score.reversed()
    } else {
        child_node.score()
    };

    let threads = f64::from(child_node.threads()) as f32;
    if threads > 0.0 {
        let v = f64::from(child_visits) as f32;
        let w = (score.win_chance() * v) / (v + threads);
        let d = (score.draw_chance() * v) / (v + threads);
        score = WDLScore::new(w, d)
    }

    score
}

fn get_cpuct(options: &EngineOptions, parent_node: &Node) -> f64 {
    let mut cpuct = options.cpuct();

    let visit_scale = options.cpuct_visit_scale();
    cpuct *= 1.0 + ((f64::from(parent_node.visits()) + visit_scale) / visit_scale).ln();

    if parent_node.visits() > 1 {
        let var = (parent_node.squared_score() - (parent_node.score().single(0.5) as f64).powi(2)).max(0.0);
        let variance = var.sqrt() / options.cpuct_variance_scale();
        cpuct *= 1.0 + options.cpuct_variance_weight() * (variance - 1.0);
    }

    cpuct *= (options.gini_base() - options.gini_scale() * (parent_node.gini_impurity() as f64 + 0.001).ln()).min(options.gini_min());

    cpuct
}
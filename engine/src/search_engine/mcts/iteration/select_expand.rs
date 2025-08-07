use chess::{ChessPosition, ZobristKey};

use crate::{SearchEngine, WDLScore};

impl SearchEngine {
    pub(super) fn select_and_expand(&self, position: &mut ChessPosition, selection_stack: &mut Vec<(usize, ZobristKey)>, castle_mask: &[u8; 64]) -> Option<usize> {
        let mut node_idx = self.tree().root_index();

        selection_stack.push((node_idx, position.board().hash()));

        loop {
            let parent_node = self.tree().get_node(node_idx);
            let parent_score = parent_node.score();
            node_idx = self.tree().select_child_by_key(node_idx, |child_node| {
                let visits = child_node.visits();

                let mut score = if visits == 0 {
                    parent_score.reversed()
                } else {
                    child_node.score()
                };

                let threads = child_node.threads() as f32;
                if threads > 0.0 {
                    let v: f32 = visits as f32;
                    let w = (score.win_chance() * v) / (v + threads);
                    let d = (score.draw_chance() * v) / (v + threads);
                    score = WDLScore::new(w, d)
                }

                let score = score.single(0.5);

                let mut cpuct = self.options().cpuct();

                let visit_scale = self.options().cpuct_visit_scale();
                cpuct *= 1.0 + ((parent_node.visits() as f64 + visit_scale) / visit_scale).ln();

                puct(score as f64, cpuct, self.tree().get_node(node_idx).visits(), child_node.visits(), child_node.policy())
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

fn puct(score: f64, cpuct: f64, parent_visits: u32, child_visits: u32, policy: f64) -> f64 {
    score + cpuct * policy * (f64::from(parent_visits.max(1)).sqrt() / f64::from(child_visits + 1))
}
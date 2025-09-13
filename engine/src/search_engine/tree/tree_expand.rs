use chess::ChessBoard;

use crate::{search_engine::engine_options::EngineOptions, NodeIndex, PolicyNetwork, Tree};

impl Tree {
    pub fn expand_node(&self, node_idx: NodeIndex, depth: f64, board: &ChessBoard, engine_options: &EngineOptions) -> Option<()> {
        let mut children_idx = self[node_idx].children_index_mut();

        if self[node_idx].children_count() > 0 {
            return Some(());
        }

        assert_eq!(
            self[node_idx].children_count(),
            0,
            "Node {node_idx} already have children."
        );

        let policy_inputs = PolicyNetwork.get_inputs(board);
        let mut policy_cache: [Option<Vec<f32>>; 192] = [const { None }; 192];

        let pst = calculate_pst(engine_options, self[node_idx].score().single(0.5), depth);

        let mut moves = Vec::new();
        let mut policy = Vec::with_capacity(board.occupancy().pop_count() as usize);
        let mut max = f64::NEG_INFINITY;
        let mut total = 0f64;

        board.map_legal_moves(|mv| {
            moves.push(mv);
            let p = PolicyNetwork.forward(board, &policy_inputs, mv, &mut policy_cache) as f64;
            policy.push(p);
            max = max.max(p);
        });

        let start_index = self.current_half().reserve_nodes(moves.len())?;

        for p in policy.iter_mut() {
            *p = ((*p - max)/pst).exp();
            total += *p;
        }

        *children_idx = start_index;
        self[node_idx].set_children_count(moves.len());

        for (idx, mv) in moves.into_iter().enumerate() {
            let p = if policy.len() == 1 {
                1.0
            } else {
                policy[idx] / total
            };

            self[start_index + idx].clear(mv);
            self[start_index + idx].set_policy(p as f64);
        }

        Some(())
    }
}

fn calculate_pst(options: &EngineOptions, parent_score: f64, depth: f64) -> f64 {
    let scalar = parent_score - parent_score.min(options.winning_pst_threshold());
    let t = scalar / (1.0 - options.winning_pst_threshold());
    let base_pst = 1.0 - options.base_pst()
        + (depth - options.root_pst()).powf(-options.depth_pst_adjustment());
    base_pst + (options.winning_pst_max() - base_pst) * t
}
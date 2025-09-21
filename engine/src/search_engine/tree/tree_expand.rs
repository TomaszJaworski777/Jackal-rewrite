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

        let mut squares = 0.0;
        for (idx, mv) in moves.into_iter().enumerate() {
            let p = if policy.len() == 1 {
                1.0
            } else {
                policy[idx] / total
            };

            self[start_index + idx].clear(mv);
            self[start_index + idx].set_policy(p);

            squares += p * p;
        }

        let gini_impurity = (1.0 - squares).clamp(0.0, 1.0);
        self[node_idx].set_gini_impurity(gini_impurity);

        Some(())
    }

    const RELABEL_DEPTH: u8 = 2;
    pub fn relabel_root(&self, board: &ChessBoard, engine_options: &EngineOptions) {
        self.recurse_relabel(self.root_index(), Self::RELABEL_DEPTH, board, engine_options);
    }

    fn recurse_relabel(&self, node_idx: NodeIndex, depth: u8, board: &ChessBoard, engine_options: &EngineOptions) {
        if depth == 0 {
            return;
        }

        self.relabel_node(node_idx, Self::RELABEL_DEPTH + 1 - depth, board, engine_options);

        let mask = board.castle_rights().get_castle_mask();
        self[node_idx].map_children(|child_idx| {
            let mut board_copy = board.clone();
            board_copy.make_move(self[child_idx].mv(), &mask);

            self.recurse_relabel(child_idx, depth - 1, &board_copy, engine_options);
        });
    }

    fn relabel_node(&self, node_idx: NodeIndex, depth: u8, board: &ChessBoard, engine_options: &EngineOptions) {
        let children_idx = *self[node_idx].children_index();

        if self[node_idx].children_count() == 0 {
            return;
        }

        let policy_inputs = PolicyNetwork.get_inputs(board);
        let mut policy_cache: [Option<Vec<f32>>; 192] = [const { None }; 192];

        let pst = calculate_pst(engine_options, self[node_idx].score().single(0.5), depth as f64);

        let mut policy = Vec::with_capacity(board.occupancy().pop_count() as usize);
        let mut max = f64::NEG_INFINITY;
        let mut total = 0f64;

        self[node_idx].map_children(|child_idx| {
            let mv = self[child_idx].mv();
            let p = PolicyNetwork.forward(board, &policy_inputs, mv, &mut policy_cache) as f64;
            policy.push(p);
            max = max.max(p);
        });

        for p in policy.iter_mut() {
            *p = ((*p - max)/pst).exp();
            total += *p;
        }

        let mut squares = 0.0;
        for (idx, p) in policy.iter().enumerate() {
            let p = if policy.len() == 1 {
                1.0
            } else {
                p / total
            };

            self[children_idx + idx].set_policy(p as f64);
            squares += p * p;
        }

        let gini_impurity = (1.0 - squares).clamp(0.0, 1.0);
        self[node_idx].set_gini_impurity(gini_impurity);
    }
}

fn calculate_pst(options: &EngineOptions, parent_score: f64, depth: f64) -> f64 {
    let scalar = parent_score - parent_score.min(options.winning_pst_threshold());
    let t = scalar / (1.0 - options.winning_pst_threshold());
    let base_pst = 1.0 - options.base_pst()
        + (depth - options.root_pst()).powf(-options.depth_pst_adjustment());
    base_pst + (options.winning_pst_max() - base_pst) * t
}
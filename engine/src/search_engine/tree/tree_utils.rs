use std::sync::atomic::Ordering;

use chess::ChessBoard;

use crate::{search_engine::{engine_options::EngineOptions, tree::{node::Node, pv_line::PvLine, Tree}}, PolicyNetwork};

impl Tree {
    pub fn expand_node(&self, node_idx: usize, depth: f64, board: &ChessBoard, engine_options: &EngineOptions) -> bool {
        let _lock = self.write_lock(node_idx);

        if self.nodes[node_idx].children_count() > 0 {
            return true;
        }

        assert_eq!(
            self.nodes[node_idx].children_count(),
            0,
            "Node {node_idx} already have children."
        );

        let policy_inputs = PolicyNetwork.get_inputs(board);
        let mut policy_cache: [Option<Vec<f32>>; 192] = [const { None }; 192];

        let pst = calculate_pst(engine_options, self.get_node(node_idx).score().single(0.5), depth);

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

        let start_index = self.reserve_nodes(moves.len());

        if start_index + moves.len() >= self.nodes.len() {
            return false;
        }

        for p in policy.iter_mut() {
            *p = ((*p - max)/pst).exp();
            total += *p;
        }

        self.nodes[node_idx].add_children(start_index, moves.len());

        let mut policy_squares = 0.0;

        for (idx, mv) in moves.into_iter().enumerate() {
            let p = if policy.len() == 1 {
                1.0
            } else {
                policy[idx] / total
            };

            self.nodes[start_index + idx].clear(mv);
            self.nodes[start_index + idx].set_policy(p as f64);
            policy_squares += p * p;
        }

        let gini_impurity = (1.0 - policy_squares).clamp(0.0, 1.0);
        self.nodes[node_idx].set_gini_impurity(gini_impurity);

        true
    }

    #[inline]
    fn reserve_nodes(&self, count: usize) -> usize {
        self.idx.fetch_add(count, Ordering::Relaxed)
    }

    pub fn select_child_by_key<F: FnMut(&Node) -> f64>(
        &self,
        parent_idx: usize,
        mut key: F,
    ) -> Option<usize> {
        let mut best_idx = None;
        let mut best_score = f64::NEG_INFINITY;

        let _lock = self.read_lock(parent_idx);

        self.nodes[parent_idx].map_children(|child_idx| {
            let node = self.get_node(child_idx);
            let new_score = key(&node);
            if new_score > best_score {
                best_idx = Some(child_idx);
                best_score = new_score;
            }
        });

        best_idx
    }

    pub fn select_best_child(&self, parent_idx: usize) -> Option<usize> {
        self.select_child_by_key(parent_idx, |node| node.score().single(0.5) as f64)
    }

    pub fn get_pv(&self, node_idx: usize) -> PvLine {
        let node = self.get_node(node_idx);

        if node.children_count() == 0 {
            return PvLine::new(&node);
        }

        let best_child_idx = self.select_best_child(node_idx);

        if best_child_idx.is_none() {
            return PvLine::new(&node);
        }

        let best_child_idx = best_child_idx.unwrap();

        let mut result = self.get_pv(best_child_idx);
        result.add_node(&node);

        result
    }

    pub fn get_best_pv(&self, index: usize) -> PvLine {
        let mut chilren_nodes = Vec::new();
        let node = self.get_root_node();

        let _lock = self.read_lock(self.root_index());

        node.map_children(|child_idx| {
            let _child_lock = self.read_lock(child_idx);

            let node = self.get_node(child_idx);

            if node.visits() == 0 {
                return;
            }

            chilren_nodes.push((child_idx, node.score().single(0.5)))
        });

        if chilren_nodes.is_empty() {
            return PvLine::new(&Node::new());
        }

        chilren_nodes.sort_by(|(_, a), (_, b)| b.total_cmp(a));

        let (pv_node_idx, _) = chilren_nodes[index.min(chilren_nodes.len() - 1)];
        self.get_pv(pv_node_idx)
    }

    pub fn find_node_depth(&self, start_node_idx: usize, target_node_idx: usize) -> Option<u64> {

        if start_node_idx == target_node_idx {
            return Some(0);
        }

        let node = self.get_node(start_node_idx);

        let mut chilren_nodes = Vec::new();
        let mut found_node = false;

        node.map_children(|child_idx| {
            if child_idx == target_node_idx {
                found_node = true
            }

            if self.get_node(child_idx).children_count() == 0 {
                return;
            }

            chilren_nodes.push(child_idx)
        });


        if found_node {
            return Some(1);
        }

        for child_idx in chilren_nodes {
            let result = self.find_node_depth(child_idx, target_node_idx);
            if result.is_none() {
                continue;
            }

            return Some(result.unwrap() + 1);
        }

        return None;
    }
}

//Formula taken from Monty
fn calculate_pst(options: &EngineOptions, parent_score: f64, depth: f64) -> f64 {
    let scalar = parent_score - parent_score.min(options.winning_pst_threshold());
    let t = scalar / (1.0 - options.winning_pst_threshold());
    let base_pst = 1.0 - options.base_pst()
        + (depth - options.root_pst()).powf(-options.depth_pst_adjustment());
    base_pst + (options.winning_pst_max() - base_pst) * t
}
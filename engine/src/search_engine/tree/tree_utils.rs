use chess::{ChessBoard, Move};

use crate::{search_engine::{engine_options::EngineOptions, tree::{node::{Edge, Node}, pv_line::PvLine, Tree}}, PolicyNetwork};

impl Tree {
    pub fn bytes_to_size(bytes: usize) -> usize {
        bytes / (std::mem::size_of::<Node>() + 18 * std::mem::size_of::<Edge>())
    }

    pub fn size_to_bytes(size: usize) -> usize {
        size * (std::mem::size_of::<Node>() + 18 * std::mem::size_of::<Edge>())
    }

    pub fn expand_node(&self, node_idx: usize, parent_edge: &Edge, depth: f64, board: &ChessBoard, engine_options: &EngineOptions) {
        let mut children = self.get_node(node_idx).children_mut();
        
        if children.len() > 0 {
            return;
        }

        let policy_inputs = PolicyNetwork.get_inputs(board);
        let mut policy_cache: [Option<Vec<f32>>; 192] = [const { None }; 192];

        let pst = calculate_pst(engine_options, parent_edge.score().single(0.5), depth);

        let mut moves = Vec::new();
        let mut max = f64::NEG_INFINITY;
        let mut total = 0f64;

        board.map_legal_moves(|mv| {
            let p = PolicyNetwork.forward(board, &policy_inputs, mv, &mut policy_cache) as f64;
            moves.push((mv, p));
            max = max.max(p);
        });

        for (_, p) in moves.iter_mut() {
            *p = ((*p - max)/pst).exp();
            total += *p;
        }

        for &(mv, policy) in moves.iter() {
            let p = if moves.len() == 1 {
                1.0
            } else {
                policy / total
            };

            let edge = Edge::new(mv);
            edge.set_policy(p as f64);
            children.push(edge);
        }
    }

    pub fn select_child_by_key<F: FnMut(&Edge) -> f64>(
        &self,
        node_idx: usize,
        mut key: F,
    ) -> Option<usize> {
        let mut best_idx = None;
        let mut best_score = f64::NEG_INFINITY;

        for (idx, child) in self.nodes[node_idx].children().iter().enumerate() {
            let new_score = key(child);
            if new_score > best_score {
                best_idx = Some(idx);
                best_score = new_score;
            }
        }

        best_idx
    }

    pub fn select_best_child(&self, node_idx: usize) -> Option<usize> {
        self.select_child_by_key(node_idx, |child| child.score().single(0.5) as f64)
    }

    pub fn get_pv(&self, node_idx: usize) -> PvLine {
        if node_idx == usize::MAX {
            return PvLine::EMPTY;
        }

        let node = self.get_node(node_idx);

        if node.children_count() == 0 {
            return PvLine::EMPTY;
        }

        let best_child_idx = self.select_best_child(node_idx);

        if best_child_idx.is_none() {
            return PvLine::EMPTY;
        }

        let best_child_idx = best_child_idx.unwrap();

        let child = self.get_child_copy(node_idx, best_child_idx);

        let mut result = self.get_pv(child.node_index());
        result.add_mv(child.mv());
        result.set_score(child.score());
        result.set_state(node.state());

        result
    }

    pub fn get_best_pv(&self, index: usize) -> PvLine {
        let children_lock = self.get_root_node().children();
        let mut chilren = Vec::new();

        for child in children_lock.iter() {

            if child.visits() == 0 || child.node_index() == usize::MAX {
                continue;
            }

            chilren.push(child)
        }

        if chilren.is_empty() {
            return PvLine::new(Move::NULL);
        }

        chilren.sort_by(|&a, &b| b.score().single(0.5).total_cmp(&a.score().single(0.5)));

        let child = chilren[index.min(chilren.len() - 1)];
        let mut result = self.get_pv(child.node_index());
        result.add_mv(child.mv());
        result.set_score(child.score());

        result
    }

    pub fn find_node_depth(&self, start_node_idx: usize, target_node_idx: usize) -> Option<u64> {

        if start_node_idx == target_node_idx {
            return Some(0);
        }

        let children_lock = self.get_node(start_node_idx).children();
        let mut chilren = Vec::new();

        for child in children_lock.iter() {
            if child.node_index() == usize::MAX || self.get_node(child.node_index()).children_count() == 0 {
                continue;
            }

            chilren.push(child)
        }

        for child in chilren {
            let result = self.find_node_depth(child.node_index(), target_node_idx);
            if result.is_none() {
                continue;
            }

            return Some(result.unwrap() + 1);
        }

        return None;
    }
}

fn calculate_pst(options: &EngineOptions, parent_score: f64, depth: f64) -> f64 {
    let scalar = parent_score - parent_score.min(options.winning_pst_threshold());
    let t = scalar / (1.0 - options.winning_pst_threshold());
    let base_pst = 1.0 - options.base_pst()
        + (depth - options.root_pst()).powf(-options.depth_pst_adjustment());
    base_pst + (options.winning_pst_max() - base_pst) * t
}
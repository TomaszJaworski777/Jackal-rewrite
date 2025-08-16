use crate::{search_engine::engine_options::EngineOptions, Node, SearchEngine, WDLScore};

impl SearchEngine {
    pub(super) fn select(&self, node_idx: usize, depth: f64) -> usize {
        let parent_node = self.tree().get_node(node_idx);

        let cpuct = get_cpuct(&self.options(), &parent_node, depth);
        let exploration_scale = get_exploration_scale(self.options(), &parent_node);

        let expl = cpuct * exploration_scale;

        self.tree().select_child_by_key(node_idx, |child_node| {
            let score = get_score(&parent_node.score(), child_node, child_node.visits()).single(0.5) as f64;
            score + child_node.policy() * expl / f64::from(child_node.visits() + 1)
        }).expect("Failed to select a valid node.")
    }
}

fn get_score(parent_score: &WDLScore, child_node: &Node, child_visits: u32) -> WDLScore {
    let mut score = if child_visits == 0 {
        parent_score.reversed()
    } else {
        child_node.score()
    };

    let threads = f64::from(child_node.threads());
    if threads > 0.0 {
        let v = f64::from(child_visits);
        let w = (score.win_chance() * v) / (v + threads);
        let d = (score.draw_chance() * v) / (v + threads);
        score = WDLScore::new(w, d)
    }

    score
}

fn get_cpuct(options: &EngineOptions, parent_node: &Node, depth: f64) -> f64 {
    let mut cpuct = options.cpuct();

    let visit_scale = options.cpuct_visit_scale();
    cpuct *= 1.0 + ((f64::from(parent_node.visits()) + visit_scale) / visit_scale).ln();

    if parent_node.visits() > 1 {
        let var = (parent_node.squared_score() - (parent_node.score().single(0.5) as f64).powi(2)).max(0.0);
        let variance = var.sqrt() / options.cpuct_variance_scale();
        cpuct *= 1.0 + options.cpuct_variance_weight() * (variance - 1.0);
    }

    cpuct *= (1.0 - depth.ln() / options.cpuct_depth_scale()).max(options.cpuct_min_depth_mul());

    cpuct
}

fn get_exploration_scale(options: &EngineOptions, parent_node: &Node) -> f64 {
    (options.exploration_tau() * (parent_node.visits().max(1) as f64).ln()).exp()
}
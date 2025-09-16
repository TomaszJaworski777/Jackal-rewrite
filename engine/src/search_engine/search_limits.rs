use crate::{search_engine::{engine_options::EngineOptions, SearchStats}, Tree};

#[derive(Debug, Default)]
pub struct SearchLimits {
    depth: Option<u64>,
    iters: Option<u64>,
    soft_limit: Option<u128>,
    hard_limit: Option<u128>,
    infinite: bool,
}

impl SearchLimits {
    pub fn set_depth(&mut self, depth: Option<u64>) {
        self.depth = depth
    }

    pub fn set_iters(&mut self, iters: Option<u64>) {
        self.iters = iters
    }

    pub fn set_time(&mut self, time: u128) {
        self.hard_limit = Some(time);
        self.soft_limit = Some(time)
    }

    pub fn set_infinite(&mut self, infinite: bool) {
        self.infinite = infinite
    }

    pub fn is_inifinite(&self) -> bool {
        self.infinite
    }

    pub fn calculate_time_limit(
        &mut self,
        time_remaining: Option<u128>,
        increment: Option<u128>,
        moves_to_go: Option<u128>,
        options: &EngineOptions
    ) {
        let move_overhead = (options.move_overhead() + (options.threads() - 1) * 10) as u128;
        let increment = increment.unwrap_or(0);

        let time_remaining = if let Some(time_remaining) = time_remaining {
            time_remaining
        } else {
            return;
        };

        let moves_to_go = if let Some(mtg) = moves_to_go.filter(|&m| m > 0) { 
            mtg
        } else {
            options.default_moves_to_go() as u128
        };

        let (soft_limit, hard_limit) = calculate_time_base(time_remaining, increment, moves_to_go, move_overhead, options);
        self.soft_limit = Some(soft_limit);
        self.hard_limit = Some(hard_limit);
    } 

    pub fn is_limit_reached(&self, search_stats: &SearchStats) -> bool {
        if self.infinite {
            return false;
        }

        if let Some(iters) = self.iters {
            if search_stats.iterations() >= iters {
                return true;
            }
        }

        if let Some(depth) = self.depth {
            if search_stats.avg_depth() >= depth {
                return true;
            }
        }

        false
    }

    pub fn is_timeout(&self, search_stats: &SearchStats, tree: &Tree, options: &EngineOptions) -> bool {
        if self.soft_limit.is_none() || self.hard_limit.is_none() {
            return false;
        }

        let time_passed_ms = search_stats.time_passesd_ms();
        
        if time_passed_ms >= self.hard_limit.unwrap() {
            return true;
        }

        let soft_limit = self.soft_limit.unwrap();

        time_passed_ms >= (soft_limit as f64 * soft_limit_modifier(search_stats, tree, options)) as u128
    }
}

fn calculate_time_base(time_remaining: u128, increment: u128, moves_to_go: u128, move_overhead: u128, options: &EngineOptions) -> (u128, u128) {
    let soft_limit = calculate_base_soft_limit(time_remaining, increment, moves_to_go, options);
    let hard_limit = calculate_hard_limit(soft_limit, time_remaining, increment, move_overhead, options);

    (soft_limit.min(hard_limit), hard_limit)
}

fn calculate_base_soft_limit(time_remaining: u128, increment: u128, moves_to_go: u128, _options: &EngineOptions) -> u128 {
    let base_limit = time_remaining / moves_to_go + increment / 2;

    base_limit
}

fn calculate_hard_limit(soft_limit: u128, time_remaining: u128, increment: u128, move_overhead: u128, options: &EngineOptions) -> u128 {
    ((soft_limit as f64 * options.hard_limit_multi()).min((time_remaining + increment) as f64 * options.max_time_fraction()) as u128).saturating_sub(move_overhead).max(1)
}

fn soft_limit_modifier(search_stats: &SearchStats, tree: &Tree, options: &EngineOptions) -> f64 {
    let mut base_modifier = 1.0;

    base_modifier *= visits_distribution(search_stats, tree, options);

    base_modifier
}

fn visits_distribution(search_stats: &SearchStats, tree: &Tree, options: &EngineOptions) -> f64 {
    if search_stats.iterations() < 2048 {
        return 1.0;
    }

    let mut best_idx = None;
    let mut best_score = f64::NEG_INFINITY;

    let mut second_best_idx = None;
    let mut second_best_score = f64::NEG_INFINITY;

    tree[tree.root_index()].map_children(|child_idx| {
        let new_score = tree[child_idx].score().single(0.5);
        if new_score > second_best_score {
            second_best_idx = Some(child_idx);
            second_best_score = new_score;

            if second_best_score > best_score {
                (best_idx, second_best_idx) = (second_best_idx, best_idx);
                (best_score, second_best_score) = (second_best_score, best_score);
            }
        }
    });

    let best_move_visits = if let Some(best_idx) = best_idx {
        tree[best_idx].visits()
    } else {
        tree.root_node().visits()
    };

    let second_move_visits = if let Some(second_best_idx) = second_best_idx {
        tree[second_best_idx].visits()
    } else {
        0
    };

    let visit_gap_ratio = ((best_move_visits - second_move_visits) as f64).abs() / tree.root_node().visits() as f64 - options.gap_threshold();
    let visit_gap = if visit_gap_ratio > 0.0 {
        2.0 * options.gap_reward_scale() / (1.0 + (options.gap_reward_multi() * visit_gap_ratio).exp()) - options.gap_reward_scale()
    } else {
        2.0 * options.gap_penalty_scale() / (1.0 + (options.gap_penalty_multi() * visit_gap_ratio).exp()) - options.gap_penalty_scale()
    };

    let best_move_visit_ratio = best_move_visits as f64 / tree.root_node().visits() as f64;
    let visit_difference_ratio = best_move_visit_ratio - options.visit_distr_threshold();
    
    let time_multiplier = if visit_difference_ratio > 0.0 {
        let reward_scale = options.visit_reward_scale() + visit_gap;
        2.0 * reward_scale / (1.0 + (options.visit_reward_multi() * visit_difference_ratio).exp()) - reward_scale
    } else {
        let penalty_scale = options.visit_penalty_scale() + visit_gap;
        2.0 * penalty_scale / (1.0 + (options.visit_penalty_multi() * visit_difference_ratio).exp()) - penalty_scale
    };
  
    1.0 + time_multiplier
}
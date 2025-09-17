use crate::{search_engine::engine_options::EngineOptions, SearchStats, Tree};

#[derive(Debug, Default, Clone, Copy)]
pub struct TimeManager {
    soft_limit: Option<u128>,
    hard_limit: Option<u128>,
    previous_score: Option<f64>,
}

#[allow(unused)]
impl TimeManager {
    pub fn set_time(&mut self, time: u128) {
        self.hard_limit = Some(time);
        self.soft_limit = Some(time)
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

        let soft_limit = time_remaining / moves_to_go + increment / 2;
        let hard_limit = ((soft_limit as f64 * options.hard_limit_multi()).min((time_remaining + increment) as f64 * options.max_time_fraction()) as u128).saturating_sub(move_overhead).max(1);

        self.soft_limit = Some(soft_limit);
        self.hard_limit = Some(hard_limit);
    } 

    pub fn is_timeout(&mut self, search_stats: &SearchStats, tree: &Tree, options: &EngineOptions) -> bool {
        if self.soft_limit.is_none() || self.hard_limit.is_none() {
            return false;
        }

        let time_passed_ms = search_stats.time_passesd_ms();
        
        if time_passed_ms >= self.hard_limit.unwrap() {
            return true;
        }

        let mut soft_limit_multiplier = 1.0;

        soft_limit_multiplier *= self.visits_distribution(search_stats, tree, options);
        soft_limit_multiplier *= self.falling_eval(search_stats, tree, options);

        time_passed_ms >= (self.soft_limit.unwrap() as f64 * soft_limit_multiplier) as u128
    }

    fn visits_distribution(&mut self, search_stats: &SearchStats, tree: &Tree, options: &EngineOptions) -> f64 {
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

        let visit_difference_ratio = best_move_visits as f64 / tree.root_node().visits() as f64 - options.visit_distr_threshold();
        
        let time_multiplier = if visit_difference_ratio > 0.0 {
            let reward_scale = options.visit_reward_scale() + visit_gap;
            2.0 * reward_scale / (1.0 + (options.visit_reward_multi() * visit_difference_ratio).exp()) - reward_scale
        } else {
            let penalty_scale = options.visit_penalty_scale() + visit_gap;
            2.0 * penalty_scale / (1.0 + (options.visit_penalty_multi() * visit_difference_ratio).exp()) - penalty_scale
        };
    
        1.0 + time_multiplier
    }

    fn falling_eval(&mut self, search_stats: &SearchStats, tree: &Tree, options: &EngineOptions) -> f64 {
        if search_stats.iterations() < 2048 {
            return 1.0;
        }

        let current_score = tree[tree.select_best_child(tree.root_index()).unwrap()].score().cp(0.5) as f64 / 100.0;
        let score_trend = if let Some(previous_score) = self.previous_score {
            let trend = current_score - previous_score;
            self.previous_score = Some(previous_score + options.falling_eval_ema_alpha() * trend);
            trend
        } else {
            self.previous_score = Some(current_score);
            0.0
        };

        let multiplier = if score_trend > 0.0 {
            -curve(score_trend * options.falling_eval_reward_multi(), options.falling_eval_reward_power(), options.falling_eval_reward_scale())
        } else {
            curve(-score_trend * options.falling_eval_penalty_multi(), options.falling_eval_penalty_power(), options.falling_eval_penalty_scale())
        };

        let multiplier = -score_trend * 5.0;

        (1.0 + multiplier).clamp(0.6, 1.8)
    }
}

fn curve(value: f64, power: f64, scale: f64) -> f64 { //TODO: Replace sigmoids in visit ratio
    assert!(value >= 0.0);
    (value / 2.0).tanh().powf(power) * scale
}
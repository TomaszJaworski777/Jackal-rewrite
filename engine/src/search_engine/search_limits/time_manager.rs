use core::f64;

use crate::{search_engine::engine_options::EngineOptions, SearchStats, Tree};

#[derive(Debug, Default, Clone, Copy)]
pub struct TimeManager {
    soft_limit: Option<u128>,
    hard_limit: Option<u128>,
    previous_score: Option<f64>,
    previous_best_move_changes: Option<f64>
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
        options: &EngineOptions,
        game_ply: u16,
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
            30
        };

        let mtg = 30;

        let time_left = (time_remaining + increment * (mtg - 1) - 10 * (2 + mtg)).max(1) as f64;
        let log_time = (time_left / 1000.0).log10();

        let soft_constant = (0.0048 + 0.00032 * log_time).min(0.0060);
        let soft_scale = (0.0125 + (game_ply as f64 + 2.5).sqrt() * soft_constant)
            .min(0.25 * time_remaining as f64 / time_left);

        let hard_constant = (3.39 + 3.01 * log_time).max(2.93);
        let hard_scale = (hard_constant + game_ply as f64 / 12.0).min(4.00);

        let bonus = if game_ply <= 10 {
            1.0 + (11.0 - game_ply as f64).log10() * 0.5
        } else {
            1.0
        };

        let soft_time = (soft_scale * bonus * time_left) as u128;
        let hard_time = (hard_scale * soft_time as f64).min(time_remaining as f64 * 0.850) as u128;

        self.soft_limit = Some(soft_time);
        self.hard_limit = Some(hard_time);
    } 

    pub fn hard_limit_reached(&mut self, search_stats: &SearchStats) -> bool {
        if self.soft_limit.is_none() || self.hard_limit.is_none() {
            return false;
        }

        search_stats.time_passesd_ms() >= self.hard_limit.unwrap()
    }

    pub fn soft_limit_reached(&mut self, search_stats: &SearchStats, tree: &Tree, options: &EngineOptions, best_move_changes: usize) -> bool {
        if self.soft_limit.is_none() || self.hard_limit.is_none() {
            return false;
        }

        let move_overhead = (options.move_overhead() + (options.threads() - 1) * 10) as u128;
        let time_passed_ms = search_stats.time_passesd_ms();
        
        let mut soft_limit_multiplier = 1.0;

        soft_limit_multiplier *= self.visits_distribution(search_stats, tree, options);
        soft_limit_multiplier *= self.falling_eval(search_stats, tree, options);
        soft_limit_multiplier *= self.best_move_instability(search_stats, tree, options, best_move_changes);

        time_passed_ms >= ((self.soft_limit.unwrap() as f64 * soft_limit_multiplier) as u128).saturating_sub(move_overhead).max(1)
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

        let visit_gap_ratio = ((best_move_visits.saturating_sub(second_move_visits)) as f64).abs() / tree.root_node().visits() as f64 - options.gap_threshold();

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

        let multiplier = curve(score_trend + 0.5, options.falling_eval_power(), options.falling_eval_multi()).clamp(-options.falling_reward_clamp(), options.falling_penalty_clamp());

        1.0 + multiplier
    }

    fn best_move_instability(&mut self, search_stats: &SearchStats, tree: &Tree, options: &EngineOptions, best_move_changes: usize) -> f64 {
        if search_stats.iterations() < 2048 {
            return 1.0;
        }

        let move_changes = if let Some(mut previous_changes) = self.previous_best_move_changes {
            previous_changes += options.instability_ema_alpha() * (best_move_changes as f64 - previous_changes);
            self.previous_best_move_changes = Some(previous_changes);
            previous_changes
        } else {
            self.previous_best_move_changes = Some(best_move_changes as f64);
            best_move_changes as f64
        };

        let multiplier = (options.instability_multi() * move_changes).powi(2).min(1.0) * options.instability_scale();

        1.0 + multiplier
    }

    fn when_behind(&mut self, search_stats: &SearchStats, tree: &Tree, options: &EngineOptions) -> f64 {
        if search_stats.iterations() < 1024 {
            return 1.0;
        }

        let current_score = tree[tree.select_best_child(tree.root_index()).unwrap()].score().cp(0.5) as f64 / 100.0;

        if current_score >= 0.0 {
            return 1.0;
        }

        let multiplier = (options.behind_multi() * -current_score).powi(2).min(1.0) * options.behind_scale();

        1.0 + multiplier
    }
}

fn curve(value: f64, power: f64, scale: f64) -> f64 {
    let ln = (1.0 / value.clamp(0.0, 1.0) - 1.0).ln();
    ln.abs().powf(power).copysign(ln) * scale
}
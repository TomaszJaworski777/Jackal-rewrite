use crate::search_engine::{engine_options::EngineOptions, SearchStats};

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
        let move_overhead = (options.move_overhead() + options.threads() * 10) as u128;
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

    pub fn is_timeout(&self, search_stats: &SearchStats) -> bool {
        if self.soft_limit.is_none() || self.hard_limit.is_none() {
            return false;
        }

        let time_passed_ms = search_stats.time_passesd_ms();
        
        if time_passed_ms >= self.hard_limit.unwrap() {
            return true;
        }

        let soft_limit = self.soft_limit.unwrap();

        time_passed_ms >= soft_limit
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
use crate::search_engine::SearchStats;


#[derive(Debug, Default)]
pub struct SearchLimits {
    depth: Option<u64>,
    iters: Option<u64>,
    time: Option<u128>,
    infinite: bool
}

impl SearchLimits {
    pub fn set_depth(&mut self, depth: Option<u64>) {
        self.depth = depth
    }

    pub fn set_iters(&mut self, iters: Option<u64>) {
        self.iters = iters
    }

    pub fn set_time(&mut self, time: u128) {
        self.time = Some(time)
    }

    pub fn set_infinite(&mut self, infinite: bool) {
        self.infinite = infinite
    }

    pub fn is_inifinite(&self) -> bool {
        self.infinite
    }

    pub fn calculate_time_limit(&mut self, time_remaining: Option<u128>, increment: Option<u128>, moves_to_go: Option<u128>) {
        if time_remaining.is_none() {
            return;
        }

        if let Some(moves_to_go) = moves_to_go {
            self.time = Some(time_remaining.unwrap_or(0) / moves_to_go);
            return;
        }

        let time = time_remaining.unwrap_or(0) / 40 + increment.unwrap_or(0) / 2;
        self.time = Some(time)
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
        if self.time.is_none() {
            return false;
        }

        let time_passed_ms = search_stats.time_passesd();
        time_passed_ms >= self.time.unwrap()
    }
}
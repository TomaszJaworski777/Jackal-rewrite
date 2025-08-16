use std::{thread, time::Instant};

use crate::{
    search_engine::{SearchLimits, SearchStats},
    SearchEngine, SearchReport,
};

mod iteration;

impl SearchEngine {
    pub(super) fn mcts<Display: SearchReport>(&self, search_limits: &SearchLimits) -> SearchStats {
        let castle_mask = self
            .current_position()
            .board()
            .castle_rights()
            .get_castle_mask();

        let search_stats = SearchStats::new(0);

        thread::scope(|s| {
            s.spawn(|| {
                self.main_loop::<Display>(&search_stats, &search_limits, &castle_mask);
            });

            for _ in 0..(self.options().threads() - 1) {
                s.spawn(|| {
                    while !self.is_search_interrupted() {
                        let _ = self.search_loop(&search_stats, search_limits, &castle_mask);
                    }
                });
            }
        });

        search_stats
    }

    fn main_loop<Display: SearchReport>(
        &self,
        search_stats: &SearchStats,
        search_limits: &SearchLimits,
        castle_mask: &[u8; 64],
    ) {
        let mut search_report_timer = Instant::now();
        let mut max_avg_depth = 0;

        while !self.is_search_interrupted() {
            if !self.search_loop(search_stats, search_limits, castle_mask) {
                break;
            }

            if search_stats.avg_depth() > max_avg_depth || search_report_timer.elapsed().as_secs_f64() > 1.0 / Display::refresh_rate_per_second() {
                Display::search_report(search_limits, search_stats, self);
                search_report_timer = Instant::now();
                max_avg_depth = max_avg_depth.max(search_stats.avg_depth());
            }

            if search_stats.iterations() % 128 != 0 {
                continue;
            }

            if search_limits.is_timeout(search_stats) {
                self.interrupt_search();
            }
        }
    }

    fn search_loop(        
        &self,         
        search_stats: &SearchStats,
        search_limits: &SearchLimits,
        castle_mask: &[u8; 64],
    ) -> bool {
        let mut depth = 0.0;
        let mut position = *self.current_position();

        if self.perform_iteration::<true>(self.tree().root_index(), &mut position, &mut depth, castle_mask).is_none() {
            if search_limits.is_inifinite() {
                while !self.is_search_interrupted() {}
            } else {
                self.interrupt_search();
            }

            return false;
        }

        search_stats.add_iteration(depth as u64);

        if search_limits.is_limit_reached(search_stats) {
            self.interrupt_search();
        }

        true
    }
}

use std::{thread, time::Instant};

use crate::{
    search_engine::{SearchLimits, SearchStats},
    SearchEngine, SearchReport,
};

mod iteration;

impl SearchEngine {
    pub(super) fn mcts<Display: SearchReport>(&self, search_limits: &SearchLimits) -> SearchStats {
        let castle_mask = self
            .root_position()
            .board()
            .castle_rights()
            .get_castle_mask();

        let search_stats = SearchStats::new(0);

        let mut search_report_timer = Instant::now();
        let mut max_avg_depth = 0;

        loop 
        {
            thread::scope(|s| {
                s.spawn(|| {
                    self.main_loop::<Display>(&search_stats, &search_limits, &castle_mask, &mut search_report_timer, &mut max_avg_depth);
                });

                for _ in 0..(self.options().threads() - 1) {
                    s.spawn(|| {
                        self.worker_loop(&search_stats, &search_limits, &castle_mask)
                    });
                }
            });

            if self.is_search_interrupted() {
                break;
            }

            self.tree().swap_half();
        }

        search_stats
    }

    fn main_loop<Display: SearchReport>(
        &self,
        search_stats: &SearchStats,
        search_limits: &SearchLimits,
        castle_mask: &[u8; 64],
        search_report_timer: &mut Instant,
        max_avg_depth: &mut u64
    ) -> Option<()> {
        while !self.is_search_interrupted() {
            self.search_loop(search_stats, search_limits, castle_mask)?;

            if search_stats.avg_depth() > *max_avg_depth || search_report_timer.elapsed().as_secs_f64() > (1.0 / Display::refresh_rate_per_second()) {
                Display::search_report(search_limits, search_stats, self);
                *search_report_timer = Instant::now();
                *max_avg_depth = search_stats.avg_depth().max(*max_avg_depth);
            }

            if search_stats.iterations() % 256 != 0 {
                continue;
            }

            if search_limits.is_timeout(search_stats, self.tree(), self.options()) {
                self.interrupt_search();
            }
        }

        Some(())
    }

    fn worker_loop(
        &self,
        search_stats: &SearchStats,
        search_limits: &SearchLimits,
        castle_mask: &[u8; 64],
    ) -> Option<()> {
        while !self.is_search_interrupted() {
            self.search_loop(search_stats, search_limits, castle_mask)?;
        }

        Some(())
    }

    fn search_loop(        
        &self,         
        search_stats: &SearchStats,
        search_limits: &SearchLimits,
        castle_mask: &[u8; 64],
    ) -> Option<()> {
        let mut depth = 0.0;
        let mut position = *self.root_position();

        self.perform_iteration::<true>(self.tree().root_index(), &mut position, &mut depth, castle_mask)?;

        search_stats.add_iteration(depth as u64);

        if search_limits.is_limit_reached(search_stats) {
            self.interrupt_search();
        }

        Some(())
    }
}

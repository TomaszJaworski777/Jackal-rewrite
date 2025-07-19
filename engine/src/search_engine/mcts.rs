use crate::{
    search_engine::{mcts::mcts_iteration::perform_iteration, SearchLimits, SearchStats},
    SearchEngine,
};

mod mcts_iteration;

impl SearchEngine {
    pub(super) fn mcts(&self, search_limits: &SearchLimits) -> SearchStats {
        let castle_mask = self
            .current_position()
            .board()
            .castle_rights()
            .get_castle_mask();

        let mut search_stats = SearchStats::new(0);

        //schedule main loop and workers
        self.main_loop(&mut search_stats, &search_limits, &castle_mask);

        search_stats
    }

    fn main_loop(
        &self,
        search_stats: &mut SearchStats,
        search_limits: &SearchLimits,
        castle_mask: &[u8; 64],
    ) {
        while !self.is_search_interrupted() {
            let mut depth = 0;
            let mut position = *self.current_position();

            let result = perform_iteration(&self.tree, 0, &mut position, &mut depth, castle_mask);

            if result.is_none() {
                if search_limits.is_inifinite() {
                    while !self.is_search_interrupted() {}
                    break;
                } else {
                    self.interrupt_search();
                }
            }

            search_stats.push_iteration(depth);

            if search_limits.is_limit_reached(search_stats) {
                self.interrupt_search();
            }

            if search_stats.iterations() % 128 != 0 {
                continue;
            }

            if search_limits.is_timeout(search_stats) {
                self.interrupt_search();
            }
        }
    }
}

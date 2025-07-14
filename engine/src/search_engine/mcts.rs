use crate::{search_engine::{mcts::mcts_iteration::perform_iteration, SearchStats}, SearchEngine};

mod mcts_iteration;

impl SearchEngine {
    pub(super) fn mcts(&self) -> SearchStats {
        let mut search_stats = SearchStats::default();

        //schedule main loop and workers
        self.main_loop(&mut search_stats);

        search_stats
    }

    fn main_loop(&self, search_stats: &mut SearchStats) {
        loop {
            let mut depth = 0;
            let mut position = *self.current_position();
            if !perform_iteration(&self.tree, &mut position, &mut depth) {
                self.interrupt_search();
                break;
            }

            search_stats.push_iteration(depth);

            if self.is_search_interrupted() {
                break;
            }
        }
    }
}
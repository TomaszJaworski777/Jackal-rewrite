use engine::{SearchEngine, SearchLimits, SearchReport, SearchStats};

pub struct UciSearchReport;
impl SearchReport for UciSearchReport {
    fn refresh_rate_per_second() -> f64 {
        1.0
    }

    fn search_report(_: &SearchLimits, search_stats: &SearchStats, search_engine: &SearchEngine) {
        let best_node_idx = search_engine
                .tree()
                .select_child(0, |node| node.score() as f64);

        if best_node_idx.is_none() {
            return;
        }

        let best_node_idx = best_node_idx.unwrap();

        let depth = search_stats.avg_depth();
        let max_depth = search_stats.max_depth();

        let score = search_engine.tree().get_node(best_node_idx).score();

        let time = search_stats.time_passesd_ms();
        let nodes = search_stats.iterations();

        let nps = (nodes as u128 * 1000) / time.max(1);

        let hashfull = search_engine.tree().current_index() * 1000 / search_engine.tree().tree_size();

        let pv = search_engine.tree().get_node(best_node_idx).mv().to_string(false);

        println!("info depth {depth} seldepth {max_depth} score cp {score} time {time} nodes {nodes} nps {nps} hashfull {hashfull} multipv 1 pv {pv}")
    }

    fn search_ended(search_engine: &SearchEngine) {
        let best_node = search_engine
                .tree()
                .select_child(0, |node| node.score() as f64);

        println!(
            "bestmove {}",
            search_engine
                .tree()
                .get_node(best_node.unwrap())
                .mv()
                .to_string(false)
        );
    }
}
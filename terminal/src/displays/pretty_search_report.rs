use engine::{SearchEngine, SearchReport};

pub struct PrettySearchReport;
impl SearchReport for PrettySearchReport {
    fn refresh_rate_per_second() -> f64 {
        20.0
    }

    fn search_ended(search_engine: &SearchEngine) {
        let best_node = search_engine
                .tree()
                .select_child_by_key(0, |node| node.score() as f64);

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
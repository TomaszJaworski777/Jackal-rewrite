use engine::{SearchEngine, SearchLimits, SearchReport, SearchStats};

pub struct UciSearchReport;
impl SearchReport for UciSearchReport {
    fn refresh_rate_per_second() -> f64 {
        1.0
    }

    fn search_report(_: &SearchLimits, search_stats: &SearchStats, search_engine: &SearchEngine) {
        let depth = search_stats.avg_depth();
        let max_depth = search_stats.max_depth();

        let pv_count = search_engine.tree().get_root_node().children_count().min(search_engine.options().multi_pv() as u8);

        for pv_idx in 0..pv_count {
            let pv = search_engine.tree().get_best_pv(pv_idx as usize);

            let state = pv.first_node().state();
            let score = match state {
                engine::GameState::Loss(len) => format!("mate {}", (len + 1).div_ceil(2)),
                engine::GameState::Win(len) => format!("mate -{}", (len + 1).div_ceil(2)),
                _ => format!("cp {}", pv.score().cp(0.5))
            };

            let wdl = if search_engine.options().show_wdl() {
                format!(" wdl {:.0} {:.0} {:.0}", 
                    pv.score().win_chance() * 1000.0, 
                    pv.score().draw_chance() * 1000.0,
                    pv.score().lose_chance() * 1000.0
                )
            } else {
               String::new() 
            };
            
            let time = search_stats.time_passesd_ms();
            let nodes = if search_engine.options().report_iters() {
                search_stats.iterations()
            } else {
                search_stats.cumulative_depth()
            };

            let nps = (nodes as u128 * 1000) / time.max(1);

            let hashfull = search_engine.tree().current_index().idx() as usize * 1000 / search_engine.tree().max_size();

            let pv = pv.to_string(search_engine.options().chess960());

            println!("info depth {depth} seldepth {max_depth} score {score}{wdl} time {time} nodes {nodes} nps {nps} hashfull {hashfull} multipv {} pv {pv}", pv_idx + 1)   
        }
    }

    fn search_ended(_: &SearchLimits, _: &SearchStats, search_engine: &SearchEngine) {
        let best_node_idx = search_engine.tree().select_best_child(search_engine.tree().root_index());

        if best_node_idx.is_none() {
            return;
        }

        println!(
            "bestmove {}",
            search_engine
                .tree()
                .get_node(best_node_idx.unwrap())
                .mv()
                .to_string(search_engine.options().chess960())
        );
    }
}
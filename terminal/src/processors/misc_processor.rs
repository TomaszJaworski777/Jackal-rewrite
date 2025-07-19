use chess::DEFAULT_PERFT_DEPTH;
use engine::SearchEngine;
use utils::{clear_terminal_screen, miliseconds_to_string, number_to_string};

pub struct MiscProcessor;
impl MiscProcessor {
    pub fn execute(
        command: &str,
        args: &[String],
        search_engine: &mut SearchEngine,
        shutdown_token: &mut bool,
    ) -> bool {
        match command {
            "exit" | "quit" | "q" => *shutdown_token = true,
            "clear" | "clean" | "cls" => clear_terminal_screen(),
            "draw" | "d" => search_engine.current_position().board().draw_board(),
            "tree" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u8>().ok()
                } else {
                    None
                };
                let node_idx = if args.len() >= 2 {
                    usize::from_str_radix(
                        args[1].strip_prefix("0x").unwrap_or(args[1].as_str()),
                        16,
                    )
                    .ok()
                } else {
                    None
                };
                search_engine.tree().draw_tree::<true>(depth, node_idx);
            }
            "rawtree" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u8>().ok()
                } else {
                    None
                };
                let node_idx = if args.len() >= 2 {
                    usize::from_str_radix(
                        args[1].strip_prefix("0x").unwrap_or(args[1].as_str()),
                        16,
                    )
                    .ok()
                } else {
                    None
                };
                search_engine.tree().draw_tree::<false>(depth, node_idx);
            }
            "perft" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u8>().ok()
                } else {
                    None
                };
                perft::<false, false>(search_engine, depth);
            }
            "bulk" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u8>().ok()
                } else {
                    None
                };
                perft::<true, false>(search_engine, depth);
            }
            "bench" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u64>().ok()
                } else {
                    None
                };
                let (result, duration) = search_engine.bench(depth);
                let nps = result as f64 / duration.as_secs_f64();
                println!("Bench: {result} nodes {:.0} nps", nps);
            }
            _ => return false,
        }

        true
    }
}

fn perft<const BULK: bool, const CHESS_960: bool>(search_engine: &SearchEngine, depth: Option<u8>) {
    println!("");

    search_engine.current_position().board().draw_board();

    println!("-----------------------------------------------------------");
    println!("  Running PERFT");
    println!("  Depth: {}", depth.unwrap_or(DEFAULT_PERFT_DEPTH));
    println!("  Bulk: {BULK}");
    println!("-----------------------------------------------------------\n");

    let (result, duration) = chess::perft::<BULK, true, CHESS_960>(
        search_engine.current_position().board(),
        depth
    );
    let miliseconds = duration.as_millis();

    println!("\n-----------------------------------------------------------");
    println!(
        "  Perft ended! {result} nodes, {}, {}n/s",
        miliseconds_to_string(miliseconds),
        number_to_string(((result * 1000) as f64 / miliseconds as f64) as u128)
    );
    println!("-----------------------------------------------------------\n");
}

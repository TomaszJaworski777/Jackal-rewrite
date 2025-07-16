use chess::DEFAULT_PERFT_DEPTH;
use engine::SearchEngine;
use utils::{clear_terminal_screen, miliseconds_to_string, number_to_string};

pub struct MiscProcessor;
impl MiscProcessor {
    pub fn execute(command: &str, args: &[String], search_engine: &mut SearchEngine, shutdown_token: &mut bool) -> bool {
        match command {
            "exit" | "quit" | "q" => *shutdown_token = true,
            "clear" | "clean" | "cls" => clear_terminal_screen(),
            "draw" | "d" => search_engine.current_position().board().draw_board(),
            "tree" => {
                let depth = if args.len() >= 1 { args[0].parse::<u8>().ok() } else { None };
                let node_idx = if args.len() >= 2 { 
                    usize::from_str_radix(args[1].strip_prefix("0x").unwrap_or(args[1].as_str()), 16).ok() 
                } else { None };
                search_engine.tree().draw_tree(depth, node_idx);
            },
            "perft" => {
                let depth = if args.len() >= 1 { args[0].parse::<u8>().ok() } else { None };
                perft(search_engine, depth, false);
            },
            "bulk" => {
                let depth = if args.len() >= 1 { args[0].parse::<u8>().ok() } else { None };
                perft(search_engine, depth, true);
            },
            _ => return false,
        }

        true
    }
}

fn perft(search_engine: &SearchEngine, depth: Option<u8>, bulk: bool) {
    println!("");

    search_engine.current_position().board().draw_board();

    println!("-----------------------------------------------------------");
    println!("  Running PERFT");
    println!("  Depth: {}", depth.unwrap_or(DEFAULT_PERFT_DEPTH));
    println!("  Bulk: {bulk}");
    println!("-----------------------------------------------------------\n");

    let (result, duration) = chess::perft(search_engine.current_position().board(), depth, bulk, false, true);
    let miliseconds = duration.as_millis();

    println!("\n-----------------------------------------------------------");
    println!(
        "  Perft ended! {result} nodes, {}, {}n/s",
        miliseconds_to_string(miliseconds),
        number_to_string(((result * 1000) as f64 / miliseconds as f64) as u128)
    );
    println!("-----------------------------------------------------------\n");
}

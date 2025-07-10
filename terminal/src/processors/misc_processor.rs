use engine::SearchEngine;
use utils::clear_terminal_screen;

pub struct MiscProcessor;
impl MiscProcessor {
    pub fn execute(command: &str, args: &[String], search_engine: &mut SearchEngine, shutdown_token: &mut bool) -> bool {
        match command {
            "exit" | "quit" | "q" => *shutdown_token = true,
            "clear" | "clean" | "cls" => clear_terminal_screen(),
            "perft" => {
                let depth = if args.len() >= 1 { args[0].parse::<u8>().unwrap_or(5) } else { 5 };
                perft(search_engine, depth, false);
            },
            "bulk" => {
                let depth = if args.len() >= 1 { args[0].parse::<u8>().unwrap_or(5) } else { 5 };
                perft(search_engine, depth, true);
            },
            _ => return false,
        }

        true
    }
}

fn perft(search_engine: &SearchEngine, depth: u8, bulk: bool) {
    
}

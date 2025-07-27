use chess::{ChessBoard, DEFAULT_PERFT_DEPTH, FEN};
use engine::{SearchEngine, ValueNetwork};
use utils::{clear_terminal_screen, miliseconds_to_string, number_to_string, CustomColor, Theme, DRAW_COLOR, LOSE_COLOR, WIN_COLOR};

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
            },
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
            },
            "perft" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u8>().ok()
                } else {
                    None
                };
                perft::<false, false>(search_engine, depth);
            },
            "bulk" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u8>().ok()
                } else {
                    None
                };
                perft::<true, false>(search_engine, depth);
            },
            "bench" => {
                let depth = if args.len() >= 1 {
                    args[0].parse::<u64>().ok()
                } else {
                    None
                };
                let (result, duration) = search_engine.bench(depth);
                let nps = result as f64 / duration.as_secs_f64();
                println!("Bench: {result} nodes {:.0} nps", nps);
            },
            "eval" => {
                let board = search_engine.current_position().board();

                board.draw_board();

                let wdl_score = ValueNetwork.forward(board);
                println!("{}",
                    format!("Raw: {}", 
                        format!("[{}, {}, {}]",
                            format!("{:.2}%", wdl_score.win_chance() * 100.0).custom_color(WIN_COLOR),
                            format!("{:.2}%", wdl_score.draw_chance() * 100.0).custom_color(DRAW_COLOR),
                            format!("{:.2}%", wdl_score.lose_chance() * 100.0).custom_color(LOSE_COLOR),
                        ).secondary(10.0/18.0)
                    ).primary(10.0/18.0)
                )
            },
            "eval-bench" => {
                const FENS: [&str; 5] = [
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
                ];

                for fen in FENS {
                    let board = ChessBoard::from(&FEN::from(fen));
                    let wdl_score = ValueNetwork.forward(&board);
                    println!("{}",
                        format!("{fen}: {}", 
                            format!("[{}, {}, {}]",
                                format!("{:.2}%", wdl_score.win_chance() * 100.0).custom_color(WIN_COLOR),
                                format!("{:.2}%", wdl_score.draw_chance() * 100.0).custom_color(DRAW_COLOR),
                                format!("{:.2}%", wdl_score.lose_chance() * 100.0).custom_color(LOSE_COLOR),
                            ).secondary(10.0/18.0)
                        ).primary(10.0/18.0)
                    )
                }
            },
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

    let (result, duration) =
        chess::perft::<BULK, true, CHESS_960>(search_engine.current_position().board(), depth);
    let miliseconds = duration.as_millis();

    println!("\n-----------------------------------------------------------");
    println!(
        "  Perft ended! {result} nodes, {}, {}n/s",
        miliseconds_to_string(miliseconds),
        number_to_string(((result * 1000) as f64 / miliseconds as f64) as u128)
    );
    println!("-----------------------------------------------------------\n");
}

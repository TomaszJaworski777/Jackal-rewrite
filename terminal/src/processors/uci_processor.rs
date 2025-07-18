use chess::{ChessBoard, ChessPosition, Side, FEN};
use engine::{SearchEngine, SearchLimits};
use utils::clear_terminal_screen;

use crate::InputWrapper;

pub struct UciProcessor {
    uci_initialized: bool,
}

impl UciProcessor {
    pub fn new() -> Self {
        Self {
            uci_initialized: false,
        }
    }

    pub fn execute(
        &mut self,
        command: &str,
        args: &[String],
        search_engine: &mut SearchEngine,
        input_wrapper: &mut InputWrapper,
        shutdown_token: &mut bool,
    ) -> bool {
        match command {
            "uci" => uci(search_engine),
            "isready" => println!("readyok"),
            "ucinewgame" => search_engine.reset_position(),
            "setoption" => set_option(args, search_engine),
            "position" => position(args, search_engine),
            "go" => go(args, search_engine, input_wrapper, shutdown_token),
            _ => return false,
        }

        true
    }
}

fn uci(search_engine: &SearchEngine) {
    clear_terminal_screen();

    println!("id name Jackal v{}", env!("CARGO_PKG_VERSION"));
    println!("id author Tomasz Jaworski");

    //options

    println!("uciok");
}

fn set_option(args: &[String], search_engine: &mut SearchEngine) {}

fn position(args: &[String], search_engine: &mut SearchEngine) {
    let mut move_flag = false;
    let mut fen_flag = false;
    let mut fen = String::new();
    let mut moves = Vec::new();

    for arg in args {
        match arg.as_str() {
            "startpos" => fen = String::from(FEN::start_position()),
            "fen" => {
                fen_flag = true;
                move_flag = false;
            }
            "moves" => {
                fen_flag = false;
                move_flag = true;
            }
            _ => {
                if fen_flag {
                    fen.push_str(&format!("{arg} "))
                }

                if move_flag {
                    moves.push(arg)
                }
            }
        }
    }

    if !FEN::validate_fen(&fen) {
        println!("Provided fen is invalid.");
        return;
    }

    let mut chess_position = ChessPosition::from(ChessBoard::from(&FEN::from(fen)));
    for mv in moves {
        chess_position.board().clone().map_legal_moves(|legal_mv| {
            if *mv == legal_mv.to_string(false) {
                chess_position.make_move(legal_mv);
            }
        });
    }

    search_engine.set_position(&chess_position);
    println!("Position has been set.");
}

fn go(
    args: &[String],
    search_engine: &mut SearchEngine,
    input_wrapper: &mut InputWrapper,
    shutdown_token: &mut bool,
) {
    let search_limits = create_search_limits(args, search_engine.current_position().board());

    std::thread::scope(|s| {
        s.spawn(|| {
            let result = search_engine.search(&search_limits);
            let best_node = search_engine
                .tree()
                .select_child(0, |node| node.score() as f64);
            println!("avg_depth {}", result.avg_depth());
            println!("iters {}", result.iterations());
            println!(
                "bestmove {}",
                search_engine
                    .tree()
                    .get_node(best_node.unwrap())
                    .mv()
                    .to_string(false)
            );
        });

        loop {
            let input_command = input_wrapper.get_input_no_queue();

            match input_command.trim() {
                "isready" => println!("readyok"),
                "stop" => search_engine.interrupt_search(),
                "quit" => {
                    search_engine.interrupt_search();
                    *shutdown_token = true
                }
                _ => {
                    if search_engine.is_search_interrupted() {
                        input_wrapper.push_back(input_command)
                    }
                }
            }

            if search_engine.is_search_interrupted() {
                break;
            }
        }
    });
}

fn create_search_limits(args: &[String], board: &ChessBoard) -> SearchLimits {
    let mut search_limits = SearchLimits::default();

    let mut iters = None;
    let mut depth = None;
    let mut move_time = None;
    let mut moves_to_go = None;
    let (mut wtime, mut btime, mut winc, mut binc) = (None, None, None, None);
    let mut infinite = false;

    for (idx, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "infinite" => infinite = true,
            "nodes" => {
                iters = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u64>().ok()
                } else {
                    None
                }
            }
            "depth" => {
                depth = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u64>().ok()
                } else {
                    None
                }
            }
            "movetime" => {
                move_time = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u128>().ok()
                } else {
                    None
                }
            }
            "moves_to_go" => {
                moves_to_go = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u128>().ok()
                } else {
                    None
                }
            }
            "wtime" => {
                wtime = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u128>().ok()
                } else {
                    None
                }
            }
            "btime" => {
                btime = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u128>().ok()
                } else {
                    None
                }
            }
            "winc" => {
                winc = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u128>().ok()
                } else {
                    None
                }
            }
            "binc" => {
                binc = if args.len() > idx + 1 {
                    args[idx + 1].parse::<u128>().ok()
                } else {
                    None
                }
            }
            _ => continue,
        }
    }

    search_limits.set_iters(iters);
    search_limits.set_depth(depth);
    search_limits.set_infinite(infinite);

    if let Some(move_time) = move_time {
        search_limits.set_time(move_time);
        return search_limits;
    }

    let (time_remaining, increment) = if board.side() == Side::WHITE {
        (wtime, winc)
    } else {
        (btime, binc)
    };

    search_limits.calculate_time_limit(time_remaining, increment, moves_to_go);

    search_limits
}

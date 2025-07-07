mod base_structures;
mod attacks;
mod board;
mod move_gen;

use std::time::Instant;

pub use base_structures::Bitboard;
pub use base_structures::Move;
pub use base_structures::MoveFlag;
pub use base_structures::Piece;
pub use base_structures::Square;
pub use base_structures::Side;
pub use base_structures::FEN;
pub use attacks::Attacks;
pub use board::ChessBoard;
pub use board::ChessPosition;

pub fn perft(fen: &FEN, depth: u8, bulk: bool, chess960: bool, print_split: bool) -> (u128, u128) {
    let board = ChessBoard::from(fen);
    let timer = Instant::now();
    let result = perft_internal(&board, depth, bulk, chess960, print_split);
    let duration = timer.elapsed().as_millis();

    (result, duration)
}

fn perft_internal(board: &ChessBoard, depth: u8, bulk: bool, chess960: bool, print_split: bool) -> u128 {
    let mut node_count = 0u128;

    if bulk && depth == 1 {
        board.map_legal_moves(|_| node_count += 1);
        return node_count;
    }

    if depth == 0 {
        return 1;
    }

    board.map_legal_moves(|mv| {
        let mut board_copy = *board;
        board_copy.make_move(mv);
        let result = perft_internal(&board_copy, depth - 1, bulk, chess960, false);
        node_count += result;

        if print_split {
            println!("{} - {result}", mv.to_string(chess960))
        }
    });

    node_count
}
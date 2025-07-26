use chess::{ChessBoard, Piece, Side};

use crate::networks::WDLScore;

#[derive(Debug)]
pub struct ValueNetwork {

}

impl ValueNetwork {
    pub const fn new() -> Self {
        Self {

        }
    }

    pub fn forward(&self, board: &ChessBoard) -> WDLScore {
        let win_chance = sigmoid(calculate_material(board));
        WDLScore::new(win_chance, 0.0)
    }
}

fn sigmoid(x: i32) -> f32 {
    1.0 / (1.0 + f32::exp(-x as f32 / 450.0))
}

fn calculate_material(board: &ChessBoard) -> i32 {
    let mut result = 0;
    
    const PIECE_VALUES: [i32; 6] = [100, 300, 330, 500, 1000, 0];

    for side in [Side::WHITE, Side::BLACK] {
        for piece_idx in u8::from(Piece::PAWN)..=u8::from(Piece::KING) {
            let piece = Piece::from(piece_idx);
            let piece_count = board.piece_mask_for_side(piece, side).pop_count();
            result += piece_count as i32 * PIECE_VALUES[piece_idx as usize];
        }

        result = -result;
    }

    result * if board.side() == Side::WHITE { 1 } else { -1 }
}
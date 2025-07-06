use utils::PieceColors;

use crate::{board::ChessBoard, Piece, Side, Square, FEN};

impl ChessBoard {
    pub fn draw_board(&self) {
        let piece_icons: [[&str; 6]; 2] = [
            [" P", " N", " B", " R", " Q", " K"],
            [" p", " n", " b", " r", " q", " k"],
        ];

        let mut info = Vec::new();
        let fen = format!("FEN: {}", FEN::from(self));
        info.push(fen.as_str());
        let zobrist = format!("Zobrist Key: {}", self.hash());
        info.push(zobrist.as_str());

        let castle_rights = format!("Castle Rights: {}", self.castle_rights());
        info.push(castle_rights.as_str());
        let side_sign = format!("Side To Move: {}", self.side());
        info.push(side_sign.as_str());
        let en_passant = format!("En Passant: {}", self.en_passant_square());
        info.push(en_passant.as_str());
        let half_moves = format!("Half Moves: {}", self.half_moves());
        info.push(half_moves.as_str());
        let in_check = format!("In Check: {}", self.is_in_check());
        info.push(in_check.as_str());
        let insufficient_material =
            format!("Insufficient material: {}", self.is_insufficient_material());
        info.push(insufficient_material.as_str());

        let mut result = " -----------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|".to_string().as_str();
            for file in 0..8 {
                let square = Square::from_coords(rank, file);
                if square == self.en_passant_square() {
                    result += " x";
                    continue;
                }

                let piece_type = self.get_piece_on_square(square);
                let piece_side = self.get_color_on_square(square);
                if piece_type == Piece::NONE {
                    result += " .";
                } else if piece_side == Side::BLACK {
                    result += piece_icons[usize::from(Side::BLACK)][usize::from(piece_type)]
                        .black_pieces()
                        .as_str();
                } else {
                    result += piece_icons[usize::from(Side::WHITE)][usize::from(piece_type)]
                        .white_pieces()
                        .as_str();
                }
            }
            result += format!(" | {}", info[(7 - rank) as usize]).as_str();
            result += "\n".to_string().as_str();
        }
        result += " -----------------\n".to_string().as_str();
        println!("{}", result);
    }
}
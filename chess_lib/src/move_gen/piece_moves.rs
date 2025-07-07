use crate::{move_gen::generate_moves::MoveGen, Attacks, Bitboard, ChessBoard, Move, MoveFlag, Piece, Side};

pub(super) const KNIGHT: u8 = 0;
pub(super) const BISHOP: u8 = 1;
pub(super) const ROOK: u8 = 2;

impl MoveGen {
    pub fn generate_piece_moves<F: FnMut(Move), const COLOR: u8, const PIECE_TYPE: u8, const CAPTURE_ONLY: bool>(board: &ChessBoard, push_map: Bitboard, capture_map: Bitboard, diagonal_pins: Bitboard, orthographpic_pins: Bitboard, apply_move: &mut F) {
        let pieces = match PIECE_TYPE {
            KNIGHT => {
                board.get_piece_mask_for_side(Piece::KNIGHT, Side::from(COLOR))
                    & !diagonal_pins
                    & !orthographpic_pins
            }
            BISHOP => {
                (board.get_piece_mask_for_side(Piece::BISHOP, Side::from(COLOR))
                    | board.get_piece_mask_for_side(Piece::QUEEN, Side::from(COLOR)))
                    & !orthographpic_pins
            }
            ROOK => {
                (board.get_piece_mask_for_side(Piece::ROOK, Side::from(COLOR))
                    | board.get_piece_mask_for_side(Piece::QUEEN, Side::from(COLOR)))
                    & !diagonal_pins
            }
            _ => unreachable!(),
        };

        let pinned_pieces = match PIECE_TYPE {
            KNIGHT => Bitboard::EMPTY,
            BISHOP => pieces & diagonal_pins,
            ROOK => pieces & orthographpic_pins,
            _ => unreachable!(),
        };

        let not_pinned_pieces = pieces & !pinned_pieces;

        not_pinned_pieces.map(|piece_square| {
            let attacks = match PIECE_TYPE {
                KNIGHT => Attacks::get_knight_attacks(piece_square),
                BISHOP => {
                    Attacks::get_bishop_attacks(piece_square, board.get_occupancy())
                }
                ROOK => {
                    Attacks::get_rook_attacks(piece_square, board.get_occupancy())
                }
                _ => unreachable!(),
            };

            (attacks & capture_map).map(|to_square| apply_move(Move::from_squares(piece_square, to_square, MoveFlag::CAPTURE)));

            if CAPTURE_ONLY {
                return;
            }

            (attacks & push_map).map(|to_square| apply_move(Move::from_squares(piece_square, to_square, MoveFlag::QUIET_MOVE)));
        });

        pinned_pieces.map(|piece_square| {
            let attacks = match PIECE_TYPE {
                KNIGHT => Bitboard::EMPTY,
                BISHOP => {
                    Attacks::get_bishop_attacks(piece_square, board.get_occupancy())
                        & diagonal_pins
                }
                ROOK => {
                    Attacks::get_rook_attacks(piece_square, board.get_occupancy())
                        & orthographpic_pins
                }
                _ => unreachable!(),
            };

            (attacks & capture_map).map(|to_square| apply_move(Move::from_squares(piece_square, to_square, MoveFlag::CAPTURE)));

            if CAPTURE_ONLY {
                return;
            }

            (attacks & push_map).map(|to_square| apply_move(Move::from_squares(piece_square, to_square, MoveFlag::QUIET_MOVE)));
        });
    }
}
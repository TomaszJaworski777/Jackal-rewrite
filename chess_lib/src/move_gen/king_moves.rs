use crate::{attacks::Rays, base_structures::CastleRights, move_gen::generate_moves::{MoveGen, WHITE}, Attacks, Bitboard, ChessBoard, Move, MoveFlag, Side, Square};

impl MoveGen {
    pub fn generate_king_moves<F: FnMut(Move), const COLOR: u8, const CAPTURE_ONLY: bool>(
        board: &ChessBoard,
        attack_map: Bitboard,
        king_square: Square,
        apply_move: &mut F,
    ) {
        let move_mask = Attacks::get_king_attacks(king_square) & !attack_map;

        (move_mask & board.get_occupancy_for_side(Side::from(COLOR).flipped()))
            .map(|square| apply_move(Move::from_squares(king_square, square, MoveFlag::CAPTURE)));

        if CAPTURE_ONLY {
            return;
        }

        (move_mask & !board.get_occupancy()).map(|square| {
            apply_move(Move::from_squares(
                king_square,
                square,
                MoveFlag::QUIET_MOVE,
            ))
        });
    }

    pub fn generate_castle_moves<F: FnMut(Move), const COLOR: u8>(
        board: &ChessBoard,
        attack_map: Bitboard,
        king_square: Square,
        apply_move: &mut F,
    ) {
        let king_side_destination = (Bitboard::from(king_square) << 2).ls1b_square();
        let queen_side_destination = (Bitboard::from(king_square) >> 2).ls1b_square();

        if COLOR == WHITE {
            let king_side_room = Rays::get_ray(king_square, board.castle_rights().rook_square(1) >> 1) & board.get_occupancy();
            let queen_side_room = Rays::get_ray(king_square, board.castle_rights().rook_square(0) << 1) & board.get_occupancy();
            if board.castle_rights().has_right(CastleRights::WHITE_KING)
                && (Rays::get_ray(king_square, king_side_destination) & attack_map).is_empty()
                && king_side_room.is_empty()
            {
                apply_move(Move::from_squares(
                    king_square,
                    king_side_destination,
                    MoveFlag::KING_SIDE_CASTLE,
                ))
            }
            if board.castle_rights().has_right(CastleRights::WHITE_QUEEN)
                && (Rays::get_ray(king_square, queen_side_destination) & attack_map).is_empty()
                && queen_side_room.is_empty()
            {
                apply_move(Move::from_squares(
                    king_square,
                    queen_side_destination,
                    MoveFlag::QUEEN_SIDE_CASTLE,
                ))
            }
        } else {
            let king_side_room = Rays::get_ray(king_square, board.castle_rights().rook_square(3) >> 1) & board.get_occupancy();
            let queen_side_room = Rays::get_ray(king_square, board.castle_rights().rook_square(2) << 1) & board.get_occupancy();
            if board.castle_rights().has_right(CastleRights::BLACK_KING)
                && (Rays::get_ray(king_square, king_side_destination) & attack_map).is_empty()
                && king_side_room.is_empty()
            {
                apply_move(Move::from_squares(
                    king_square,
                    king_side_destination,
                    MoveFlag::KING_SIDE_CASTLE,
                ))
            }
            if board.castle_rights().has_right(CastleRights::BLACK_QUEEN)
                && (Rays::get_ray(king_square, queen_side_destination) & attack_map).is_empty()
                && queen_side_room.is_empty()
            {
                apply_move(Move::from_squares(
                    king_square,
                    queen_side_destination,
                    MoveFlag::QUEEN_SIDE_CASTLE,
                ))
            }
        }
    }
}
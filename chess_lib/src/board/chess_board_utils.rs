use crate::{board::ChessBoard, Attacks, Bitboard, Piece, Side, Square};

impl ChessBoard {
    pub fn is_insufficient_material(&self) -> bool {
        let phase = self.phase();
        let bishops = self.get_piece_mask(Piece::BISHOP);
        phase <= 2
            && self.get_piece_mask(Piece::PAWN).is_empty()
            && ((phase != 2)
                || (bishops & self.get_occupancy_for_side(Side::WHITE) != bishops
                    && bishops & self.get_occupancy_for_side(Side::BLACK) != bishops
                    && (bishops & 0x55AA55AA55AA55AA == bishops
                        || bishops & 0xAA55AA55AA55AA55 == bishops)))
    }

    pub fn all_attackers_to_square(&self, occupancy: Bitboard, square: Square, defender_side: Side) -> Bitboard {
        let queens = self.get_piece_mask(Piece::QUEEN);
        ((Attacks::get_knight_attacks_for_square(square) & self.get_piece_mask(Piece::KNIGHT))
            | (Attacks::get_king_attacks_for_square(square) & self.get_piece_mask(Piece::KING))
            | (Attacks::get_pawn_attacks_for_square(square, defender_side)
                & self.get_piece_mask(Piece::PAWN))
            | (Attacks::get_rook_attacks_for_square(square, occupancy)
                & (self.get_piece_mask(Piece::ROOK) | queens))
            | (Attacks::get_bishop_attacks_for_square(square, occupancy)
                & (self.get_piece_mask(Piece::BISHOP) | queens)))
            & self.get_occupancy_for_side(defender_side.flipped())
    }

    #[inline]
    pub fn is_square_attacked_with_occupancy(&self, square: Square, occupancy: Bitboard, defender_side: Side) -> bool {
        self.all_attackers_to_square(occupancy, square, defender_side).is_not_empty()
    }

    #[inline]
    pub fn is_square_attacked(&self, square: Square, defender_side: Side) -> bool {
        self.is_square_attacked_with_occupancy(square, self.get_occupancy(), defender_side)
    }

    #[inline]
    pub fn is_in_check(&self) -> bool {
        self.is_square_attacked(self.get_king_square(self.side()), self.side())
    }
}
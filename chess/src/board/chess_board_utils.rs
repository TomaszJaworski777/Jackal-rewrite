use crate::{attacks::Rays, board::ChessBoard, Attacks, Bitboard, Piece, Side, Square};

impl ChessBoard {
    pub fn is_insufficient_material(&self) -> bool {
        let phase = self.phase();
        let bishops = self.piece_mask(Piece::BISHOP);
        phase <= 2
            && self.piece_mask(Piece::PAWN).is_empty()
            && ((phase != 2)
                || bishops.pop_count() == 2
                    && (bishops & 0x55AA55AA55AA55AA == bishops
                        || bishops & 0xAA55AA55AA55AA55 == bishops))
    }

    pub fn all_attackers_to_square(
        &self,
        occupancy: Bitboard,
        square: Square,
        defender_side: Side,
    ) -> Bitboard {
        let queens = self.piece_mask(Piece::QUEEN);
        ((Attacks::get_knight_attacks(square) & self.piece_mask(Piece::KNIGHT))
            | (Attacks::get_king_attacks(square) & self.piece_mask(Piece::KING))
            | (Attacks::get_pawn_attacks(square, defender_side) & self.piece_mask(Piece::PAWN))
            | (Attacks::get_rook_attacks(square, occupancy)
                & (self.piece_mask(Piece::ROOK) | queens))
            | (Attacks::get_bishop_attacks(square, occupancy)
                & (self.piece_mask(Piece::BISHOP) | queens)))
            & self.occupancy_for_side(defender_side.flipped())
    }

    #[inline]
    pub fn is_square_attacked_with_occupancy(
        &self,
        square: Square,
        occupancy: Bitboard,
        defender_side: Side,
    ) -> bool {
        self.all_attackers_to_square(occupancy, square, defender_side)
            .is_not_empty()
    }

    #[inline]
    pub fn is_square_attacked(&self, square: Square, defender_side: Side) -> bool {
        self.is_square_attacked_with_occupancy(square, self.occupancy(), defender_side)
    }

    #[inline]
    pub fn is_in_check(&self) -> bool {
        self.is_square_attacked(self.king_square(self.side()), self.side())
    }

    #[inline]
    pub fn generate_checkers_mask(&self, defender_side: Side) -> Bitboard {
        self.all_attackers_to_square(
            self.occupancy(),
            self.king_square(defender_side),
            defender_side,
        )
    }

    pub fn generate_pin_masks(&self, defender_side: Side) -> (Bitboard, Bitboard) {
        let king_square = self.king_square(defender_side);
        let defender_occupancy = self.occupancy_for_side(defender_side);
        let attacker_occupancy = self.occupancy_for_side(defender_side.flipped());
        let queens = self.piece_mask_for_side(Piece::QUEEN, defender_side.flipped());

        let potential_pinners = Attacks::get_bishop_attacks(king_square, attacker_occupancy)
            & (self.piece_mask_for_side(Piece::BISHOP, defender_side.flipped()) | queens);

        let mut diag_result = Bitboard::EMPTY;
        potential_pinners.map(|potential_pinner| {
            let ray = Rays::get_ray(king_square, potential_pinner);
            if (ray & defender_occupancy).only_one_bit() {
                diag_result |= ray;
            }
        });

        let potential_pinners = Attacks::get_rook_attacks(king_square, attacker_occupancy)
            & (self.piece_mask_for_side(Piece::ROOK, defender_side.flipped()) | queens);
        let mut orto_result = Bitboard::EMPTY;
        potential_pinners.map(|potential_pinner| {
            let ray = Rays::get_ray(king_square, potential_pinner);
            if (ray & defender_occupancy).only_one_bit() {
                orto_result |= ray;
            }
        });

        (diag_result, orto_result)
    }

    pub fn generate_attack_map(&self, attacker_side: Side) -> Bitboard {
        let mut threats = Bitboard::EMPTY;

        let king_square = self.king_square(attacker_side.flipped());
        let occupancy = self.occupancy() ^ Bitboard::from(king_square);

        let attacker_pieces = self.occupancy_for_side(attacker_side);
        let queens = self.piece_mask(Piece::QUEEN);

        (attacker_pieces & (self.piece_mask(Piece::ROOK) | queens))
            .map(|rook_square| threats |= Attacks::get_rook_attacks(rook_square, occupancy));

        (attacker_pieces & (self.piece_mask(Piece::BISHOP) | queens))
            .map(|bishop_square| threats |= Attacks::get_bishop_attacks(bishop_square, occupancy));

        (attacker_pieces & self.piece_mask(Piece::KING))
            .map(|king_square| threats |= Attacks::get_king_attacks(king_square));

        (attacker_pieces & self.piece_mask(Piece::KNIGHT))
            .map(|knight_square| threats |= Attacks::get_knight_attacks(knight_square));

        (attacker_pieces & self.piece_mask(Piece::PAWN))
            .map(|pawn_square| threats |= Attacks::get_pawn_attacks(pawn_square, attacker_side));

        threats
    }
}

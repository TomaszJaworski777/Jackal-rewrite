use crate::{attacks::{BishopAttacks, KingAttacks, KnightAttacks, PawnsAttacks, RookAttacks}, base_structures::Side, Bitboard, Square};

pub struct Attacks;
impl Attacks {
    #[inline]
    pub fn get_king_attacks_for_square(square: Square) -> Bitboard {
        KingAttacks::ATTACK_TABLE[usize::from(square)]
    }

    #[inline]
    pub fn get_knight_attacks_for_square(square: Square) -> Bitboard {
        KnightAttacks::ATTACK_TABLE[usize::from(square)]
    }

    #[inline]
    pub fn get_pawn_attacks_for_square(square: Square, side: Side) -> Bitboard {
        PawnsAttacks::ATTACK_TABLE[usize::from(side)][usize::from(square)]
    }

    #[inline]
    pub fn get_bishop_attacks_for_square(square: Square, occupancy: Bitboard) -> Bitboard {
        BishopAttacks::get_bishop_attacks(square, occupancy)
    }

    #[inline]
    pub fn get_rook_attacks_for_square(square: Square, occupancy: Bitboard) -> Bitboard {
        RookAttacks::get_rook_attacks(square, occupancy)
    }
}
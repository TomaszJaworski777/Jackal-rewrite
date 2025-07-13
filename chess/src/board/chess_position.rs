use crate::{
    board::{chess_board::ChessBoard, move_history::MoveHistory},
    Move,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ChessPosition {
    board: ChessBoard,
    history: MoveHistory,
}

impl ChessPosition {
    #[inline]
    pub fn board(&self) -> &ChessBoard {
        &self.board
    }

    #[inline]
    pub fn history(&self) -> &MoveHistory {
        &self.history
    }

    #[inline]
    pub fn reset_history(&mut self) {
        self.history.reset()
    }

    #[inline]
    pub fn make_move(&mut self, mv: Move) {
        self.board.make_move(mv);

        if self.board.half_moves() == 0 {
            self.history.reset()
        }

        self.history.push(self.board.hash())
    }
}

impl From<ChessBoard> for ChessPosition {
    fn from(value: ChessBoard) -> Self {
        Self {
            board: value,
            history: MoveHistory::default(),
        }
    }
}

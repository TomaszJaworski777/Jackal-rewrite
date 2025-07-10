use chess_lib::{ChessBoard, ChessPosition, FEN};

#[derive(Debug, Clone)]
pub struct SearchEngine {
    position: ChessPosition,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self { 
            position: ChessPosition::from(ChessBoard::from(&FEN::start_position())),
        }
    }
}

impl SearchEngine {
    pub fn current_position(&self) -> &ChessPosition {
        &self.position
    }

    pub fn set_position(&mut self, position: &ChessPosition) {
        self.position = *position;
    }
}
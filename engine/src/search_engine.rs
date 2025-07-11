use std::{sync::atomic::{AtomicBool, Ordering}, time::Duration};

use chess_lib::{ChessBoard, ChessPosition, FEN};

use crate::search::SearchResult;

#[derive(Debug)]
pub struct SearchEngine {
    position: ChessPosition,
    interruption_token: AtomicBool
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self { 
            position: ChessPosition::from(ChessBoard::from(&FEN::start_position())),
            interruption_token: AtomicBool::new(false),
        }
    }
}

impl Clone for SearchEngine {
    fn clone(&self) -> Self {
        Self { 
            position: self.position, 
            interruption_token: AtomicBool::new(self.interruption_token.load(Ordering::Relaxed)) 
        }
    }
}

impl SearchEngine {
    #[inline]
    pub fn current_position(&self) -> &ChessPosition {
        &self.position
    }

    #[inline]
    pub fn set_position(&mut self, position: &ChessPosition) {
        self.position = *position;
    }

    #[inline]
    pub fn reset_position(&mut self) {
        self.position = ChessPosition::from(ChessBoard::from(&FEN::start_position()))
    }

    #[inline]
    pub fn interrupt_search(&self) {
        self.interruption_token.store(true, Ordering::Relaxed)
    }

    #[inline]
    pub fn is_search_interrupted(&self) -> bool {
        self.interruption_token.load(Ordering::Relaxed)
    }

    pub fn search(&self) -> SearchResult {
        self.interruption_token.store(false, Ordering::Relaxed);

        self.interrupt_search();
        SearchResult::default()
    }
}
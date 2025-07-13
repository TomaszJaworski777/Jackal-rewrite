use std::sync::atomic::{AtomicBool, Ordering};

use chess::{ChessBoard, ChessPosition, FEN};

use crate::{SearchResult, tree::Tree};

#[derive(Debug)]
pub struct SearchEngine {
    position: ChessPosition,
    tree: Tree,
    interruption_token: AtomicBool,
}

impl Clone for SearchEngine {
    fn clone(&self) -> Self {
        Self { 
            position: self.position,
            tree: self.tree.clone(), 
            interruption_token: AtomicBool::new(self.interruption_token.load(Ordering::Relaxed)) 
        }
    }
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            position: ChessPosition::from(ChessBoard::from(&FEN::start_position())),
            tree: Tree::from_bytes(1024 * 1024 * 1024),
            interruption_token: AtomicBool::new(false),
        }
    }

    #[inline]
    pub fn current_position(&self) -> &ChessPosition {
        &self.position
    }

    #[inline]
    pub fn set_position(&mut self, position: &ChessPosition) {
        self.position = *position;
        self.tree.clear();
    }

    #[inline]
    pub fn reset_position(&mut self) {
        self.position = ChessPosition::from(ChessBoard::from(&FEN::start_position()));
        self.tree.clear();
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

        self.tree.clear();

        if self.tree.root_node().children_count() == 0 {
            self.tree.expand_node(0, self.current_position().board());
        }

        //search loop

        self.interrupt_search();
        SearchResult::default()
    }
}
use std::sync::atomic::{AtomicBool, Ordering};

use chess::{ChessBoard, ChessPosition, FEN};

use crate::search_engine::tree::Tree;

mod bench;
mod mcts;
mod search_limits;
mod search_stats;
mod tree;

pub use search_limits::SearchLimits;
pub use search_stats::SearchStats;

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
            interruption_token: AtomicBool::new(self.interruption_token.load(Ordering::Relaxed)),
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
    pub fn tree(&self) -> &Tree {
        &self.tree
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

    pub fn search(&self, search_limits: &SearchLimits) -> SearchStats {
        self.interruption_token.store(false, Ordering::Relaxed);

        self.tree.clear();

        if self.tree.root_node().children_count() == 0 {  //TEMP: It should be replaced by tree reuse code
            self.tree.expand_node(0, self.current_position().board());
        }

        self.mcts(search_limits)
    }
}

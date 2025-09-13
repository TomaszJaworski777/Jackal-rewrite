use std::sync::atomic::{AtomicBool, Ordering};

use chess::{ChessBoard, ChessPosition, FEN};

use crate::{search_engine::engine_options::EngineOptions, search_report_trait::SearchReport};

mod bench;
mod mcts;
mod search_limits;
mod search_stats;
mod tree;
mod engine_options;
mod hash_table;

pub use search_limits::SearchLimits;
pub use search_stats::SearchStats;
pub use tree::{Tree, Node, GameState, AtomicWDLScore, WDLScore, PvLine, NodeIndex};

#[derive(Debug)]
pub struct SearchEngine {
    position: ChessPosition,
    tree: Tree,
    options: EngineOptions,
    interruption_token: AtomicBool,
}

impl Clone for SearchEngine {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            tree: self.tree.clone(),
            options: self.options.clone(),
            interruption_token: AtomicBool::new(self.interruption_token.load(Ordering::Relaxed)),
        }
    }
}

impl SearchEngine {
    pub fn new() -> Self {
        let options = EngineOptions::new();

        Self {
            position: ChessPosition::from(ChessBoard::from(&FEN::start_position())),
            tree: Tree::from_bytes(options.hash() as usize, options.hash_size()),
            options,
            interruption_token: AtomicBool::new(false),
        }
    }

    #[inline]
    pub fn root_position(&self) -> &ChessPosition {
        &self.position
    }

    #[inline]
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    #[inline]
    pub fn resize_tree(&mut self) {
        self.tree = Tree::from_bytes(self.options.hash() as usize, self.options().hash_size())
    }

    #[inline]
    pub fn options(&self) -> &EngineOptions {
        &self.options
    }

    #[inline]
    pub fn set_option(&mut self, name: &str, value: &str) -> Result<(), String> {
        self.options.set_option(name, value)
    }

    #[inline]
    pub fn set_position(&mut self, position: &ChessPosition) {
        self.position = *position;
    }

    #[inline]
    pub fn reset_position(&mut self) {
        self.position = ChessPosition::from(ChessBoard::from(&FEN::start_position()));
    }

    #[inline]
    pub fn interrupt_search(&self) {
        self.interruption_token.store(true, Ordering::Relaxed)
    }

    #[inline]
    pub fn is_search_interrupted(&self) -> bool {
        self.interruption_token.load(Ordering::Relaxed)
    }

    pub fn search<Display: SearchReport>(&self, search_limits: &SearchLimits) -> SearchStats {
        self.interruption_token.store(false, Ordering::Relaxed);

        if self.tree().root_node().children_count() == 0 {
            self.tree().expand_node(self.tree().root_index(), 1.0, self.root_position().board(), self.options());
        }

        Display::search_started(search_limits, self);

        let result = self.mcts::<Display>(search_limits);

        Display::search_report(search_limits, &result, self);
        Display::search_ended(search_limits, &result, self);

        result
    }
}

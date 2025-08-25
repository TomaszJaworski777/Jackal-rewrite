use chess::Move;

use crate::{search_engine::tree::node::WDLScore, GameState};

#[derive(Debug, Clone, Default)]
pub struct PvLine(Vec<Move>, WDLScore, GameState);
impl PvLine {
    pub const EMPTY: Self = Self(Vec::new(), WDLScore::LOSE, GameState::Ongoing);

    #[inline]
    pub fn new(mv: Move) -> Self {
        Self(vec![mv], WDLScore::default(), GameState::Ongoing)
    }

    #[inline]
    pub fn add_mv(&mut self, mv: Move) {
        self.0.insert(0, mv);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn first_move(&self) -> Move {
        if self.0.len() == 0 {
            return Move::NULL;
        }

        self.0[0]
    }

    #[inline]
    pub fn score(&self) -> WDLScore {
        self.1
    }

    #[inline]
    pub fn state(&self) -> GameState {
        self.2
    }

    #[inline]
    pub fn set_score(&mut self, score: WDLScore) {
        self.1 = score
    }

    #[inline]
    pub fn set_state(&mut self, state: GameState) {
        self.2 = state
    }

    #[inline]
    pub fn to_string(&self, chess960: bool) -> String {
        let mut result = String::new();
        for mv in &self.0 {
            result.push_str(mv.to_string(chess960).as_str());
            result.push(' ');
        }

        result.trim().to_string()
    }

    #[inline]
    pub fn to_string_wrapped(&self, wrap_length: usize, chess960: bool) -> String {
        let mut result = String::new();
        for (idx, mv) in self.0.iter().enumerate() {
            if idx == wrap_length - 1 && idx < self.0.len() - 1 {
                result.push_str(format!("({} more...)", self.0.len() - wrap_length + 1).as_str());
                break;
            }

            result.push_str(mv.to_string(chess960).as_str());
            result.push(' ');
        }

        result.trim().to_string()
    }
}
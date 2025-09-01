use std::{fmt::Display, sync::atomic::{AtomicU8, Ordering}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Draw,
    Win(u8),
    Loss(u8)
}

impl Default for GameState {
    fn default() -> Self {
        Self::Ongoing
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::Ongoing => "Ongoing".to_string(),
            Self::Draw => "Draw".to_string(),
            Self::Win(x) => format!("Win in {x}"),
            Self::Loss(x) => format!("Loss in {x}"),
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, Default)]
pub struct AtomicGameState {
    state: AtomicU8,
    payload: AtomicU8
}

impl Clone for AtomicGameState {
    fn clone(&self) -> Self {
        Self { 
            state: AtomicU8::new(self.state.load(Ordering::Relaxed)), 
            payload: AtomicU8::new(self.payload.load(Ordering::Relaxed))
        }
    }
}

impl AtomicGameState {
    pub fn new(state: GameState) -> Self {
        let result = AtomicGameState::default();
        result.set(state);
        result
    }   

    pub fn set(&self, state: GameState) {
        match state {
            GameState::Ongoing => self.state.store(0, Ordering::Relaxed),
            GameState::Draw => self.state.store(1, Ordering::Relaxed),
            GameState::Win(len) => {
                self.state.store(2, Ordering::Relaxed);
                self.payload.store(len, Ordering::Relaxed);
            },
            GameState::Loss(len) => {
                self.state.store(4, Ordering::Relaxed);
                self.payload.store(len, Ordering::Relaxed);
            },
        }
    }

    pub fn get(&self) -> GameState {
        let state = self.state.load(Ordering::Relaxed);
        let payload = self.payload.load(Ordering::Relaxed);

        match state {
            0 => GameState::Ongoing,
            1 => GameState::Draw,
            2 => GameState::Win(payload),
            4 => GameState::Loss(payload),
            _ => unreachable!()
        }
    }
}
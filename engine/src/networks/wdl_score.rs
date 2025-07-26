use std::sync::atomic::{AtomicU64, Ordering};

const SCORE_SCALE: u32 = 1024 * 64;

#[derive(Debug, Default)]
pub struct AtomicWDLScore(AtomicU64, AtomicU64);

impl Clone for AtomicWDLScore {
    fn clone(&self) -> Self {
        Self(
            AtomicU64::new(self.0.load(Ordering::Relaxed)), 
            AtomicU64::new(self.1.load(Ordering::Relaxed))
        )
    }
}

impl From<WDLScore> for AtomicWDLScore {
    fn from(value: WDLScore) -> Self {
        let win_chance = (value.win_chance() as f64 * f64::from(SCORE_SCALE)) as u64;
        let draw_chance = (value.draw_chance() as f64 * f64::from(SCORE_SCALE)) as u64;

        Self(
            AtomicU64::new(win_chance), 
            AtomicU64::new(draw_chance)
        )
    }
}

impl AtomicWDLScore {
    #[inline]
    pub fn get_score(&self, visits: u32) -> WDLScore {
        let win_chance = self.0.load(Ordering::Relaxed) as f64 / f64::from(SCORE_SCALE) / f64::from(visits.max(1));
        let draw_chance = self.1.load(Ordering::Relaxed) as f64 / f64::from(SCORE_SCALE) / f64::from(visits.max(1));
        WDLScore(win_chance as f32, draw_chance as f32)
    }

    #[inline]
    pub fn clear(&self) {
        self.0.store(0, Ordering::Relaxed);
        self.1.store(0, Ordering::Relaxed);
    }

    #[inline]
    pub fn add(&self, rhs: WDLScore) {
        let win_chance = (rhs.win_chance() as f64 * f64::from(SCORE_SCALE)) as u64;
        let draw_chance = (rhs.draw_chance() as f64 * f64::from(SCORE_SCALE)) as u64;

        self.0.fetch_add(win_chance, Ordering::Relaxed);
        self.1.fetch_add(draw_chance, Ordering::Relaxed);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct WDLScore(f32, f32);
impl WDLScore {
    pub const WIN: Self = Self(1.0, 0.0);
    pub const DRAW: Self = Self(0.0, 1.0);
    pub const LOSE: Self = Self(0.0, 0.0);

    #[inline]
    pub const fn new(win_chance: f32, draw_chance: f32) -> Self {
        Self(win_chance, draw_chance)
    }

    #[inline]
    pub const fn win_chance(&self) -> f32 {
        self.0
    }

    #[inline]
    pub const fn draw_chance(&self) -> f32 {
        self.1
    }

    #[inline]
    pub const fn lose_chance(&self) -> f32 {
        1.0 - self.win_chance() - self.draw_chance()
    }

    #[inline]
    pub const fn single(&self, draw_reference: f32) -> f32 {
        self.win_chance() + self.draw_chance() * draw_reference
    }

    #[inline]
    pub const fn reversed(&self) -> Self {
        Self(1.0 - self.win_chance(), self.draw_chance())
    }
}
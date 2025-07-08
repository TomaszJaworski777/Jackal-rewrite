use crate::base_structures::ZobristKey;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveHistory([ZobristKey; 100], usize);
impl MoveHistory {
    pub fn new() -> Self {
        Self([ZobristKey::default(); 100], 0)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        let mut result = 0u64;
        for value in 0..self.1 {
            result ^= u64::from(self.0[value])
        }

        result
    }

    #[inline]
    pub fn push(&mut self, key: ZobristKey) {
        self.0[self.1] = key;
        self.1 += 1;
    }

    #[inline]
    pub fn reset(&mut self) {
        self.1 = 0;
    }

    #[inline]
    pub fn get_key_repetitions(&self, key: ZobristKey) -> i32 {
        let mut repetitions = 0;
        for value in 0..self.1 {
            if key != self.0[value] {
                continue;
            }

            repetitions += 1;
        }
        repetitions
    }
}

impl Default for MoveHistory {
    fn default() -> Self {
        Self::new()
    }
}

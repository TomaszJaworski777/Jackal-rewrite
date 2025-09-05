use crate::base_structures::ZobristKey;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveHistory([ZobristKey; 101], usize);
impl MoveHistory {
    pub fn new() -> Self {
        Self([ZobristKey::default(); 101], 0)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        let mut history_hash = 0u128;
        for value in 0..self.1 {
            let hash = (u64::from(self.0[value]) as u128) << 64 | u64::from(self.0[value]) as u128;
            history_hash ^= (hash >> value) << 7;
        }

        history_hash &= !0xFFFFFFFFFFFFFFFFFFFFFFFF0000007F;

        let mut result = history_hash as u64 | self.1 as u64;
        result |= u64::from(self.0[self.1 - 1]) << 32;
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
    pub fn get_repetitions(&self, key: ZobristKey) -> i32 {
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

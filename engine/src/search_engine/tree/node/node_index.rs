use std::{fmt::Display, ops::Add, sync::atomic::{AtomicU32, Ordering}};

#[derive(Debug)]
pub struct AtomicNodeIndex(AtomicU32);

impl Clone for AtomicNodeIndex {
    fn clone(&self) -> Self {
        Self(AtomicU32::new(self.0.load(Ordering::Relaxed)))
    }
}

impl AtomicNodeIndex {
    pub fn new(value: NodeIndex) -> Self {
        Self(AtomicU32::new(u32::from(value)))
    }

    pub fn load(&self) -> NodeIndex {
        NodeIndex::from(self.0.load(Ordering::Relaxed))
    }

    pub fn store(&self, value: NodeIndex) {
        self.0.store(u32::from(value), Ordering::Relaxed);
    }

    pub fn add(&self, value: u32) -> u32 {
        self.0.fetch_add(value, Ordering::Relaxed) as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeIndex(u32);
impl NodeIndex {
    pub const NULL: Self = Self(u32::MAX);

    pub fn new(half: u32, index: u32) -> Self {
        Self((half << 31) | (index & 0x7FFFFFFF))
    }

    pub fn is_null(&self) -> bool {
        *self == Self::NULL
    }

    pub fn half(&self) -> u32 {
        self.0 >> 31
    }

    pub fn idx(&self) -> u32 {
        self.0 & 0x7FFFFFFF
    }
}

impl From<u32> for NodeIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<NodeIndex> for u32 {
    fn from(value: NodeIndex) -> Self {
        value.0
    }
}

impl Add<u8> for NodeIndex {
    type Output = NodeIndex;

    fn add(self, rhs: u8) -> Self::Output {
        NodeIndex::new(self.half(), self.idx() + rhs as u32)
    }
}

impl Display for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = if self.is_null() {
            String::from("NULL")
        } else {
            format!("({}, {})", self.half(), self.idx())
        };

        write!(f, "{}", result)
    }
}
use std::{fmt::Display, sync::atomic::{AtomicUsize, Ordering}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeIndex(usize);
impl NodeIndex {
    pub const NULL: Self = Self(usize::MAX);

    pub fn is_null(&self) -> bool {
        self.eq(&Self::NULL)
    }
}

impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<NodeIndex> for usize {
    fn from(value: NodeIndex) -> Self {
        value.0
    }
}

impl Display for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}

#[derive(Debug)]
pub struct AtomicNodeIndex(AtomicUsize);

impl Clone for AtomicNodeIndex {
    fn clone(&self) -> Self {
        Self(AtomicUsize::new(self.0.load(Ordering::Relaxed)))
    }
}

impl AtomicNodeIndex {
    pub const fn new(value: NodeIndex) -> Self {
        Self(AtomicUsize::new(value.0))
    }

    pub fn store(&self, value: NodeIndex) {
        self.0.store(value.0, Ordering::Relaxed);
    }
}

impl From<&NodeIndex> for AtomicNodeIndex {
    fn from(value: &NodeIndex) -> Self {
        Self(AtomicUsize::new(value.0))
    }
}

impl From<&AtomicNodeIndex> for NodeIndex {
    fn from(value: &AtomicNodeIndex) -> Self {
        Self(value.0.load(Ordering::Relaxed))
    }
}
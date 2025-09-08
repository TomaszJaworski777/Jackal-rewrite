use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Node;

#[derive(Debug)]
pub struct TreeHalf {
    nodes: Vec<Node>,
    idx: AtomicUsize,
}

impl Clone for TreeHalf {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            idx: AtomicUsize::new(self.idx.load(Ordering::Relaxed)),
        }
    }
}

impl TreeHalf {
    pub fn new(size: usize) -> Self {
        Self { 
            nodes: vec![Node::new(); size], 
            idx: AtomicUsize::new(0)
        }
    }

    pub fn clear(&self) {
        self.idx.store(0, Ordering::Relaxed);
    }
}
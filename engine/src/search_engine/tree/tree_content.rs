use std::{ops::Index, sync::atomic::{AtomicUsize, Ordering}};

use crate::{Node, NodeIndex};

#[derive(Debug)]
pub struct TreeContent {
    content: Vec<Node>,
    idx: AtomicUsize
}

impl Clone for TreeContent {
    fn clone(&self) -> Self {
        Self { 
            content: self.content.clone(), 
            idx: AtomicUsize::new(self.idx.load(Ordering::Relaxed))
        }
    }
}

impl TreeContent {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            content: vec![Node::new(); size],
            idx: AtomicUsize::new(1)
        }
    }

    pub fn clear(&self) {
        self.idx.store(1, Ordering::Relaxed);
    }

    pub fn max_size(&self) -> usize {
        self.content.len()
    }

    pub fn current_size(&self) -> usize {
        self.idx.load(Ordering::Relaxed)
    }

    pub fn reserve_node(&self) -> Option<NodeIndex> {
        let node_idx = self.idx.fetch_add(1, Ordering::Relaxed);

        if node_idx + 1 >= self.max_size() {
            return None;
        }

        Some(NodeIndex::from(node_idx))
    }
}

impl Index<NodeIndex> for TreeContent {
    type Output = Node;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.content[usize::from(index)]
    }
}
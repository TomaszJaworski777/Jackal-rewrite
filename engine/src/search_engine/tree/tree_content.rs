use std::{ops::Index, sync::atomic::{AtomicUsize, Ordering}};

use crate::{Node, NodeIndex};

#[derive(Debug)]
pub struct TreeContent {
    content: Vec<Node>,
    used: AtomicUsize
}

impl Clone for TreeContent {
    fn clone(&self) -> Self {
        Self { 
            content: self.content.clone(), 
            used: AtomicUsize::new(self.used.load(Ordering::Relaxed))
        }
    }
}

impl TreeContent {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            content: vec![Node::new(); size],
            used: AtomicUsize::new(1)
        }
    }

    pub fn clear(&self) {
        self.used.store(1, Ordering::Relaxed);
    }

    pub fn max_size(&self) -> usize {
        self.content.len()
    }

    pub fn current_size(&self) -> usize {
        self.used.load(Ordering::Relaxed)
    }

    pub fn reserve_node(&self, hash: u64) -> Option<NodeIndex> {
        let node_idx = hash as usize % self.max_size();
        let mut offset = 0usize;
        while (node_idx + offset) % self.max_size() != node_idx || offset == 0 {
            if self.content[node_idx + offset].hash() == u64::MAX {
                self.used.fetch_add(1, Ordering::Relaxed);
                return Some(NodeIndex::from(node_idx + offset))
            }

            offset += 1; 
        }

        None
    }

    pub fn validate_index(&self, node_index: NodeIndex, hash: u64) -> bool {
        !node_index.is_null() && self[node_index].hash() == hash
    }
}

impl Index<NodeIndex> for TreeContent {
    type Output = Node;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.content[usize::from(index)]
    }
}
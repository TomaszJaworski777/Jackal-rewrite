use crate::{search_engine::tree::Tree, GameState, SearchEngine, WDLScore};

impl SearchEngine {
    pub(super) fn backpropagate(&self, node_idx: usize, child_idx: usize, score: WDLScore) {
        self.tree().add_visit(node_idx, child_idx, score);
        backprop_state(self.tree(), node_idx, child_idx);
    }
}

fn backprop_state(tree: &Tree, node_idx: usize, child_idx: usize) {
    let edge = &tree.get_node(node_idx).children()[child_idx];

    match tree.get_node(edge.node_index()).state() {
        GameState::Loss(len) => tree.set_state(node_idx, GameState::Win(len + 1)),
        GameState::Win(len) => {
            let mut proven_loss = true;
            let mut proven_loss_length = len;

            for child in tree.get_node(node_idx).children().iter() {
                let node_index = child.node_index();
                if node_index == usize::MAX {
                    proven_loss = false;
                    break;
                } else if let GameState::Win(len) = tree.get_node(node_index).state() {
                    proven_loss_length = proven_loss_length.max(len);
                } else {
                    proven_loss = false;
                    break;
                }
            }

            if proven_loss {
                tree.set_state(node_idx, GameState::Loss(proven_loss_length + 1));
            }
        },
        _ => (),
    }
}
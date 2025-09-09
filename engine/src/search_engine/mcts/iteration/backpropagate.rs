use chess::ZobristKey;

use crate::{search_engine::tree::{NodeIndex, Tree}, GameState, SearchEngine, WDLScore};

impl SearchEngine {
    pub(super) fn backpropagate(&self, node_idx: NodeIndex, score: WDLScore, key: ZobristKey) {
        self.tree().add_visit(node_idx, score);
        backprop_state(self.tree(), node_idx);
        self.tree().hash_table().push(key, score.reversed());
    }
}

fn backprop_state(tree: &Tree, node_idx: NodeIndex) {
    let mut proven_loss = true;
    let mut proven_loss_length = 0;

    if tree[node_idx].children_count() == 0 {
        return;
    }

    tree[node_idx].map_children(|child_idx| { //TODO: Flip this loop
        match tree[child_idx].state() {
            GameState::Loss(len) => {
                tree.set_state(node_idx, GameState::Win(len + 1));
                proven_loss = false;
            },
            GameState::Win(len) => {
                proven_loss_length = proven_loss_length.max(len)
            },
            _ => proven_loss = false,
        }
    });

    if proven_loss {
        tree.set_state(node_idx, GameState::Loss(proven_loss_length + 1));
    }
}
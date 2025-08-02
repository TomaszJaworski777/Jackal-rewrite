use crate::{search_engine::tree::Tree, GameState, SearchEngine, WDLScore};

impl SearchEngine {
    pub(super) fn backpropagate(&self, selection_stack: &Vec<usize>, mut score: WDLScore) {
        for &node_idx in selection_stack.iter().rev() {
            self.tree().dec_threads(node_idx, 1);

            self.tree().add_visit(node_idx, score);
            
            backprop_state(self.tree(), node_idx);

            score = score.reversed();
        }
    }
}

fn backprop_state(tree: &Tree, node_idx: usize) {
    let mut proven_loss = true;
    let mut proven_loss_length = 0;

    if tree.get_node(node_idx).children_count() == 0 {
        return;
    }

    let _lock = tree.read_lock(node_idx);

    tree.get_node(node_idx).map_children(|child_idx| {
        match tree.get_node(child_idx).state() {
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
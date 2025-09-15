use chess::ChessPosition;

use crate::{search_engine::tree::NodeIndex, GameState, SearchEngine, ValueNetwork, WDLScore};

impl SearchEngine {
    pub(super) fn simulate(&self, node_idx: NodeIndex, position: &ChessPosition) -> WDLScore {
        if self.tree()[node_idx].visits() == 0 {
            let state = get_node_state(position, self.root_position());
            self.tree().set_state(node_idx, state);
        }

        if self.tree[node_idx].state() == GameState::Ongoing {
            if let Some(entry) = self.tree().hash_table().get(position.board().hash()) {
                entry
            } else {
                get_position_score(position, self.tree()[node_idx].state())
            }
        } else {
            get_position_score(position, self.tree()[node_idx].state())
        }
    }
}

fn get_node_state(position: &ChessPosition, root_position: &ChessPosition) -> GameState {
    let mut possible_moves = 0;
    position.board().map_legal_moves(|_| possible_moves += 1);

    if possible_moves == 0 {
        if position.board().is_in_check() {
            GameState::Loss(0)
        } else {
            GameState::Draw
        }
    } else if is_draw(position, root_position) {
        GameState::Draw
    } else {
        GameState::Ongoing
    }
}

fn is_draw(position: &ChessPosition, root_position: &ChessPosition) -> bool {
    if position.board().half_moves() >= 100 || position.board().is_insufficient_material() {
        return true;
    }

    let key = position.board().hash();
    let history_repetitions = root_position.history().get_repetitions(key);
    let search_repetitions = position.history().get_repetitions(key) - history_repetitions;

    if history_repetitions >= 3 || search_repetitions >= 2 || history_repetitions + search_repetitions >= 3 {
        return true;
    }

    false
}

fn get_position_score(position: &ChessPosition, node_state: GameState) -> WDLScore {
    match node_state {
        GameState::Draw => WDLScore::DRAW,
        GameState::Loss(_) => WDLScore::LOSE,
        GameState::Win(_) => WDLScore::WIN,
        _ => ValueNetwork.forward(position.board())
    }
}
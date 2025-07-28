use chess::ChessPosition;

use crate::{networks::{ValueNetwork, WDLScore}, search_engine::tree::{GameState, Tree}, SearchEngine};

impl SearchEngine {
    pub(super) fn perform_iteration<const ROOT: bool>(
        &self,
        tree: &Tree,
        node_idx: usize,
        position: &mut ChessPosition,
        depth: &mut u64,
        castle_mask: &[u8; 64],
    ) -> Option<WDLScore> {  
        let score = {
            if !ROOT && (tree.get_node(node_idx).visits() == 0 || tree.get_node(node_idx).is_terminal()) {
                if tree.get_node(node_idx).visits() == 0 {
                    let state = get_node_state(position, self.current_position());
                    tree.set_state(node_idx, state);
                }

                get_position_score(position, tree.get_node(node_idx).state())
            } else {
                if tree.get_node(node_idx).children_count() == 0 {
                    if !tree.expand_node(node_idx, position.board()) {
                        return None;
                    }
                }

                let parent_score = tree.get_node(node_idx).score();
                let new_index = tree.select_child_by_key(node_idx, |node| {
                    let score = if node.visits() == 0 {
                        parent_score.reversed()
                    } else {
                        node.score()
                    }.single(0.5);


                    puct(score as f64, 2.0, tree.get_node(node_idx).visits(), node.visits(), node.policy())
                });

                assert_ne!(new_index, None);

                let new_index = new_index.unwrap();

                position.make_move(tree.get_node(new_index).mv(), castle_mask);

                *depth += 1;
                self.perform_iteration::<false>(tree, new_index, position, depth, castle_mask)?
            }
        }.reversed();

        tree.add_visit(node_idx, score);

        backprop_state(tree, node_idx);

        Some(score)
    }
}

fn puct(score: f64, c: f64, parent_visits: u32, child_visits: u32, policy: f64) -> f64 {
    score + c * policy * (f64::from(parent_visits.max(1)).sqrt() / f64::from(child_visits + 1))
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

fn get_position_score(position: &ChessPosition, node_state: GameState) -> WDLScore {
    match node_state {
        GameState::Draw => WDLScore::DRAW,
        GameState::Loss(_) => WDLScore::LOSE,
        GameState::Win(_) => WDLScore::WIN,
        _ => ValueNetwork.forward(position.board())
    }
}

fn is_draw(position: &ChessPosition, root_position: &ChessPosition) -> bool {
    if position.board().half_moves() >= 100 || position.board().is_insufficient_material() {
        return true;
    }

    let key = position.board().hash();
    let history_repetitions = root_position.history().get_repetitions(key);
    let search_repetitions = position.history().get_repetitions(key) - history_repetitions;

    if history_repetitions >=3 || search_repetitions >=2 || history_repetitions + search_repetitions >= 3 {
        return true;
    }

    false
}

fn backprop_state(tree: &Tree, node_idx: usize) {
    let mut proven_loss = true;
    let mut proven_loss_length = 0;

    if tree.get_node(node_idx).children_count() == 0 {
        return;
    }

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
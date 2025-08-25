use engine::{GameState, Node};

#[test]
fn terminal_state() {  
    let node = Node::new();

    assert!(!node.is_terminal());

    node.set_state(GameState::Draw);

    assert!(node.is_terminal());
}
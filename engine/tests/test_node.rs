use engine::{GameState, Node};

#[test]
fn terminal_state() {  
    let node = Node::new();

    assert!(!node.is_terminal());

    node.set_state(GameState::Draw);

    assert!(node.is_terminal());

    node.add_children(1, 12);

    assert!(node.is_terminal());
}

#[test]
fn map_children() {  
    let node = Node::new();

    node.add_children(1, 12);

    assert!(!node.is_terminal());

    assert_eq!(node.children_count(), 12);

    node.map_children(|child_idx| {
        assert!(child_idx >= 1 && child_idx <= 12)
    });
}
use program::*;
use rctree::Node;

#[test]
fn test() {
    let mut program = Node::new(ProgramNode::Root);
    assert_eq!("(root)", &program.inspect());

    // let
}

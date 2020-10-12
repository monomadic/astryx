
use crate::{models::AstryxNode, InterpreterResult, state::State};
use parser::Statement;
use rctree::Node;

pub(crate) fn interpret(node: &Node<Statement>, _state: State) -> InterpreterResult<Node<AstryxNode>> {
    match node.borrow().clone() {
        Statement::Element(e) => println!("# element: {:?}", e.ident.fragment()),
        Statement::FunctionCall(f) => println!("# function call: {:?}", f),
    }
    Ok(Node::new(AstryxNode::Element))
}

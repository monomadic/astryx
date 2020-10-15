
use crate::{models::AstryxNode, InterpreterResult, state::State};
use parser::Statement;
use rctree::Node;

pub(crate) fn interpret(node: &Node<Statement>, state: &State) -> InterpreterResult<Node<AstryxNode>> {
    // println!("node {:?}", node);
    match node.borrow().clone() {
        Statement::Element(e) => println!("# element: {:?}", e.ident.fragment()),
        Statement::FunctionCall(f) => println!("# function: {:?}", f.ident.fragment()),
        Statement::Text(t) => println!("# text: {:?}", t),
    }
    for child in node.children() {
        let _ = interpret(&child, state);
    }
    Ok(Node::new(AstryxNode::Element))
}

// pub(crate) fn traverse(node: &Node<Statement>, state: &State) -> InterpreterResult<Node<AstryxNode>> {


//     match node.borrow().clone() {
//         Statement::Element(e) => println!("# element: {:?}, {}", e.ident.fragment(), node.children().len()),
//         Statement::FunctionCall(f) => println!("# function call: {:?}", f),
//     }

//     let mut traversal = node.traverse();

//     while let Some(child) = traversal.next_back() {
//         match child.clone() {
//             Statement::Element(e) => println!("# element: {:?}, {}", e.ident.fragment(), node.children().len()),
//             Statement::FunctionCall(f) => println!("# function call: {:?}", f),
//         }
//     }

//     Ok(Node::new(AstryxNode::Element))
// }

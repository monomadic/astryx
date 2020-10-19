
use crate::{InterpreterResult, state::State, models::Value};
use parser::{Expression, Statement, Span};
use rctree::Node;

pub(crate) fn eval(node: &Node<Statement>, state: &mut State) -> InterpreterResult<()> {
    println!("node {:?}", node);
    match node.borrow().clone() {
        Statement::Element(e) => {
            println!("# element: {:?}", &e.ident.fragment());
            state.push_element(e)?;
        },
        Statement::Expression(expr) => { eval_expression(&expr, state); },
        Statement::Text(t) => println!("# text: {:?}", t),
        Statement::Binding(ident, expr) => { eval_binding(&ident, &expr, state); } // todo: return err
    }
    for child in node.children() {
        let _ = eval(&child, state);
    }
    // closing element node here
    Ok(())
}

fn eval_binding(ident: &Span, expr: &Expression, state: &mut State) -> InterpreterResult<()> {
    state.bind(ident.fragment(), state.eval(expr)?)
}

fn eval_expression(expr: &Expression, state: &mut State) -> InterpreterResult<Value> {
    match expr {
        Expression::FunctionCall(f) => println!("calling {:?}", f),
        _ => ()
    };

    Ok(Value::String("eval expr".into()))
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

use crate::{models::Value, state::State, InterpreterResult};
use html::HTMLElement;
use parser::{Expression, Span, Statement};
use rctree::Node;

pub(crate) fn eval(node: &Node<Statement>, state: &mut State) -> InterpreterResult<()> {
    // println!("node {:?}", node);
    match node.borrow().clone() {
        Statement::Element(e) => {
            let element = HTMLElement::new(e.ident.fragment()).expect("valid html");
            print!("{}", element.open_tag());
            state.push_element(e)?;
        }
        Statement::Expression(expr) => {
            eval_expression(&expr, state);
        }
        Statement::Text(t) => print!("{}", state.interpolate(t)?),
        Statement::Binding(ident, expr) => {
            eval_binding(&ident, &expr, state);
        } // todo: return err
    }
    for child in node.children() {
        let _ = eval(&child, state);
    }

    // closing element node here
    match node.borrow().clone() {
        Statement::Element(e) => {
            print!("</{}>", &e.ident.fragment());
        }
        _ => (),
    }

    Ok(())
}

fn eval_binding(ident: &Span, expr: &Expression, state: &mut State) -> InterpreterResult<()> {
    state.bind(ident.fragment(), state.eval(expr)?)
}

fn eval_expression(expr: &Expression, state: &mut State) -> InterpreterResult<Value> {
    match expr {
        Expression::FunctionCall(f) => println!("calling {:?}", f),
        _ => (),
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

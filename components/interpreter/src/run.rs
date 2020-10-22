use crate::{state::State, InterpreterResult};
use html::HTMLElement;
use parser::Statement;
use rctree::Node;
use std::collections::HashMap;

pub(crate) fn eval<'a>(node: &Node<Statement<'a>>, state: &mut State<'a>) -> InterpreterResult<()> {
    match node.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            for (ident, expr) in e.attributes {
                attributes.insert(ident.fragment().to_string(), state.eval(&expr)?.into());
            }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            // state.writer = Writer::StdOut;

            // let mut writer = state.get_mut_writer()?;
            // element.write_open_tag(&mut writer);

            state.write(&element.open_tag())?;

            for child in node.children() {
                let _ = eval(&child, state);
            }

            state.write(&element.close_tag())?;

            // element.write_close_tag(&mut writer);
        }
        Statement::Expression(expr) => {
            state.eval(&expr)?;
        }
        Statement::Text(t) => {
            state.write(&state.interpolate(t)?)?;
        }
        Statement::Binding(ident, expr) => {
            state.bind(ident.fragment(), expr)?;
        } // todo: return err
    }

    Ok(())
}

// fn eval_binding<'a>(
//     ident: &Span<'a>,
//     expr: Expression<'a>,
//     state: &mut State<'a>,
// ) -> InterpreterResult<()> {
//     state.bind(ident.fragment(), expr)
// }

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

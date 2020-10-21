use crate::{state::State, InterpreterResult};
use html::HTMLElement;
use parser::{Expression, Span, Statement};
use rctree::Node;
use std::collections::HashMap;

pub(crate) fn eval<'a>(node: &Node<Statement<'a>>, state: &mut State<'a>) -> InterpreterResult<()> {
    // println!("node {:?}", node);
    match node.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            for (ident, expr) in e.attributes {
                attributes.insert(ident.fragment().to_string(), state.eval(&expr)?.into());
            }

            use std::io::{self, Write};
            let stdout = std::io::stdout();
            let mut writer = stdout.lock();

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            // writer.write_all(&element.open_tag().as_bytes()).unwrap();
            element.write_open_tag(&mut writer);

            // print!("{}", element.open_tag());
            // state.push_element(e)?;
            for child in node.children() {
                let _ = eval(&child, state);
            }

            element.write_close_tag(&mut writer);
            // print!("{}", element.close_tag());
        }
        Statement::Expression(expr) => {
            state.eval(&expr)?;
        }
        Statement::Text(t) => {
            print!("{}", state.interpolate(t)?);
        }
        Statement::Binding(ident, expr) => {
            eval_binding(&ident, expr, state)?;
        } // todo: return err
    }

    Ok(())
}

// fn eval_element(e: &Element, state: &mut State) -> InterpreterResult<()> {
//     let element = HTMLElement::new(e.ident.fragment()).expect("valid html");
//     print!("{}", element.open_tag());
//     // state.push_element(e)?;
//     for child in node.children() {
//         let _ = eval(&child, state);
//     }
//     print!("{}", element.close_tag());

//     Ok(())
// }

fn eval_binding<'a>(
    ident: &Span<'a>,
    expr: Expression<'a>,
    state: &mut State<'a>,
) -> InterpreterResult<()> {
    state.bind(ident.fragment(), expr)
}

// fn eval_expression(expr: &Expression, state: &mut State) -> InterpreterResult<Value> {
//     match expr {
//         Expression::FunctionCall(f) => println!("calling {}", f.ident.fragment()),
//         Expression::Reference(_) => {}
//         Expression::Literal(_) => {}
//     };

//     Ok(Value::String("eval expr".into()))
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

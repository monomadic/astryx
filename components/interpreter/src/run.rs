use crate::{state::State, InterpreterResult};
use html::HTMLElement;
use parser::Statement;
use rctree::Node;
use std::collections::HashMap;

pub(crate) fn eval_statement<'a>(
    node: &Node<Statement<'a>>,
    state: &'a mut State<'a>,
) -> InterpreterResult<()> {
    match node.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            for (ident, expr) in e.attributes {
                attributes.insert(
                    ident.fragment().to_string(),
                    state.eval_expression(&expr)?.into(),
                );
            }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            state.write(&element.open_tag())?;

            for child in node.children() {
                let _ = eval_statement(&child, state);
            }

            state.write(&element.close_tag())?;
        }
        Statement::Expression(expr) => {
            state.eval_expression(&expr)?;
        }
        Statement::Text(t) => {
            state.write(&state.interpolate(t)?)?;
        }
        Statement::Binding(ident, expr) => {
            let obj = state.eval_expression(&expr)?;
            state.bind(ident.fragment(), obj)?;
        }
    }

    Ok(())
}

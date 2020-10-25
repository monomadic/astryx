use crate::{state::{eval_expression, State}, InterpreterResult};
use html::HTMLElement;
use parser::Statement;
use rctree::Node;
use std::{collections::HashMap, rc::Rc};
use std::cell::RefCell;

pub(crate) fn eval_statement<'a>(
    node: &Node<Statement<'a>>,
    state: Rc<RefCell<State>>,
) -> InterpreterResult<()> {
    match node.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            // for (ident, expr) in e.attributes {
            //     attributes.insert(
            //         ident.fragment().to_string(),
            //         state.eval_expression(&expr)?.into(),
            //     );
            // }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            state.borrow_mut().write(&element.open_tag())?;

            for child in node.children() {
                let _ = eval_statement(&child, state.clone());
            }

            state.borrow_mut().write(&element.close_tag())?;
        }
        Statement::Expression(expr) => {
            state.borrow_mut().eval_expression(&expr)?;
        }
        Statement::Text(t) => {
            // state.borrow_mut().write(&state.interpolate(t)?)?;
        }
        Statement::Binding(ident, expr) => {
            // let obj = state.borrow().eval_expression(&expr)?;
            // state.borrow_mut().bind(ident.fragment(), obj)?;

            // let state = state.clone().borrow_mut();
            // let obj = eval_expression(&expr)?;

            // let state = Rc::clone(&state);

            let obj = eval_expression(Rc::clone(&state), &expr)?;
            state.borrow_mut().bind(ident.fragment(), obj.clone())?;
        }
    }

    Ok(())
}

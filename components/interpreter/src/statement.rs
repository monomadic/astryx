use crate::expression;
use crate::util::span_to_location;
use error::{AstryxError, AstryxErrorKind, AstryxResult};
use html::HTMLElement;
use models::{object::Object, state::State};
use parser::{Expression, Span, Statement, StringToken};
use rctree::Node;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub(crate) fn eval_statement(
    state: Rc<RefCell<State>>,
    statement: &Node<Statement>,
) -> AstryxResult<Node<Object>> {
    match statement.borrow().clone() {
        Statement::Blank(_) => Ok(Node::new(Object::None)),
        Statement::Element(e) => {
            // let attributes = e
            //     .attributes
            //     .iter()
            //     .map(|(ident, expr)| {
            //         // todo: file references
            //         (
            //             ident.fragment().to_string(),
            //             expression::eval_expression(Rc::clone(&state), &expr, None)?.into(),
            //         )
            //     })
            //     .collect();

            let mut attributes: HashMap<String, String> = HashMap::new();

            // collect the attributes
            for (ident, expr) in e.attributes {
                // if let Expression::RelativePath(s) = expr {
                //     // println!("file reference found: {:?}", s);
                //     // todo: how can we copy?
                //     // - use state?
                //     // - add some additional information to an Object::Element?
                //     // --> attach another child object (fileref)
                //     // - don't copy for solo elements, use internal function instead
                // }
                attributes.insert(
                    ident.fragment().to_string(),
                    expression::eval_expression(Rc::clone(&state), &expr, None)?.into(),
                );
            }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            let mut node = Node::new(Object::HTMLElement(element));

            if let Some(text) = e.text {
                node.append(Node::new(Object::String(eval_interpolation(
                    Rc::clone(&state),
                    text,
                )?)));
            }

            for child in statement.children() {
                let obj = eval_statement(Rc::clone(&state), &child)?;
                node.append(obj);
            }

            Ok(node)
        }
        Statement::Expression(expr) => {
            // evaluate children first... (state??)
            // fixme: might be a bug where state is not passed to children
            let return_objects: Vec<Node<Object>> = statement
                .children()
                .map(|child| eval_statement(Rc::clone(&state), &child))
                .collect::<AstryxResult<Vec<Node<Object>>>>()?;

            let return_value = expression::eval_expression(
                Rc::clone(&state),
                &expr,
                Some(Node::new(Object::Array(return_objects))),
            )?;

            // state
            //     .borrow()
            //     .push_instruction(ProgramInstruction::Text(return_value.to_string()));

            // for child in statement.children() {
            //     eval_statement(&child, Rc::clone(&state))?;
            // }

            Ok(Node::new(return_value))
        }
        Statement::Text(t) => {
            return Ok(Node::new(Object::String(eval_interpolation(
                Rc::clone(&state),
                t,
            )?)));
        }
        Statement::Binding(ident, expr) => {
            let obj = expression::eval_expression(Rc::clone(&state), &expr, None)?;
            state.borrow_mut().bind(ident.fragment(), obj.clone())?;
            return Ok(Node::new(Object::None));
        }
        Statement::Comment(_) => Ok(Node::new(Object::None)),
        Statement::ForLoop { ident, expr } => for_loop(state, statement, ident, expr),
        Statement::Route(route) => {
            // collect attributes
            let mut attributes: HashMap<String, String> = HashMap::new();

            // collect the attributes
            for (ident, expr) in route.attributes {
                attributes.insert(
                    ident.fragment().to_string(),
                    expression::eval_expression(Rc::clone(&state), &expr, None)?.into(),
                );
            }

            match route.ident.to_string().as_str() {
                "route" => {
                    let path = attributes.get("path").ok_or(AstryxError::LocatedError(
                        span_to_location(route.ident),
                        AstryxErrorKind::MissingRequiredArgument(route.ident.to_string()),
                    ))?;

                    let mut node = Node::new(Object::HTMLPage(path.clone()));

                    for child in statement.children() {
                        // println!("child");
                        let obj = eval_statement(Rc::clone(&state), &child)?;
                        // node.append child
                        node.append(obj);
                    }

                    Ok(node)
                }
                _ => Ok(Node::new(Object::None)),
            }
        }
    }
}

/// Convert string tokens to a fully interpolated string
fn eval_interpolation(
    state: Rc<RefCell<State>>,
    components: Vec<StringToken>,
) -> AstryxResult<String> {
    Ok(components
        .into_iter()
        .map(|st| match st {
            StringToken::Text(span) => Ok(span.to_string()),
            // StringToken::Expression(expr) => self.eval(&expr).map(|e| e.into()),
            StringToken::Expression(expr) => {
                Ok(expression::eval_expression(Rc::clone(&state), &expr, None)?.to_string())
            }
        })
        .collect::<Result<Vec<String>, AstryxError>>()?
        .into_iter()
        .collect())
}

fn for_loop(
    state: Rc<RefCell<State>>,
    statement: &Node<Statement>,
    ident: Span,
    expr: Expression,
) -> AstryxResult<Node<Object>> {
    let iter: Object = expression::eval_expression(Rc::clone(&state), &expr, None)?;

    match iter {
        Object::Array(array) => {
            let mut node = Node::new(Object::None);

            for index in array {
                let childstate = state.clone();

                childstate
                    .borrow_mut()
                    .bind(&ident.to_string(), index.borrow().clone())?;

                for child in statement.children() {
                    // println!("child {:?}", eval_statement(Rc::clone(&childstate), &child));
                    node.append(eval_statement(Rc::clone(&childstate), &child)?);
                }
            }
            Ok(node)
        }
        _ => Err(AstryxError::LocatedError(
            span_to_location(ident),
            AstryxErrorKind::Unexpected,
        )),
    }
}

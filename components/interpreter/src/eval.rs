use crate::util::span_to_location;
use error::{AstryxError, AstryxErrorKind, AstryxResult};
use html::HTMLElement;
use models::{object::Object, state::State};
use parser::{Expression, Statement, StringToken};
use rctree::Node;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

pub(crate) fn eval_statement(
    statement: &Node<Statement>,
    state: Rc<RefCell<State>>,
) -> AstryxResult<Node<Object>> {
    match statement.borrow().clone() {
        Statement::Blank(_) => Ok(Node::new(Object::None)),
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            // collect the attributes
            for (ident, expr) in e.attributes {
                attributes.insert(
                    ident.fragment().to_string(),
                    eval_expression(Rc::clone(&state), &expr, None)?.into(),
                );
            }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            // todo, these really should be html nodes, so that we can optimise them all later...
            // examples:
            // - removing empty or unneeded classes/styles/ids/empty elements (optional)
            // - link rewriters
            // - searching for specific elements
            // - injection? (might need tree for this though...?)

            let mut node = Node::new(Object::HTMLElement(element));

            if let Some(text) = e.text {
                node.append(Node::new(Object::String(eval_interpolation(
                    Rc::clone(&state),
                    text,
                )?)));
            }

            for child in statement.children() {
                let obj = eval_statement(&child, Rc::clone(&state))?;
                node.append(obj);
            }

            Ok(node)
        }
        Statement::Expression(expr) => {
            let return_objects: Vec<Node<Object>> = statement
                .children()
                .map(|child| eval_statement(&child, Rc::clone(&state)))
                .collect::<AstryxResult<Vec<Node<Object>>>>()?;

            // for statement in statement.children() {

            // }

            let return_value = eval_expression(
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
            let obj = eval_expression(Rc::clone(&state), &expr, None)?;
            state.borrow_mut().bind(ident.fragment(), obj.clone())?;
            return Ok(Node::new(Object::None));
        }
        Statement::Comment(_) => Ok(Node::new(Object::None)),
        Statement::ForLoop { ident, expr } => {
            let iter: Object = eval_expression(Rc::clone(&state), &expr, None)?;

            match iter {
                Object::Array(array) => {
                    let mut node = Node::new(Object::None);

                    for index in array {
                        let childstate = state.clone();

                        childstate
                            .borrow_mut()
                            .bind(&ident.to_string(), index.borrow().clone())?;

                        for child in statement.children() {
                            node.append(eval_statement(&child, Rc::clone(&childstate))?);
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
        Statement::Route(route) => {
            // collect attributes
            let mut attributes: HashMap<String, String> = HashMap::new();

            // collect the attributes
            for (ident, expr) in route.attributes {
                attributes.insert(
                    ident.fragment().to_string(),
                    eval_expression(Rc::clone(&state), &expr, None)?.into(),
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
                        let obj = eval_statement(&child, Rc::clone(&state))?;
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

pub fn eval_expression(
    state: Rc<RefCell<State>>,
    expr: &Expression,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    match expr {
        Expression::FunctionCall(ref f) => {
            let mut inner = State::new();
            // inner.program = Rc::clone(&state.borrow().program);

            // add function arguments into scope
            for (k, expr) in &f.arguments {
                inner.bind(
                    &k.to_string(),
                    // evaluate the expression part of the argument
                    eval_expression(Rc::clone(&state), expr, None).map_err(|_| {
                        AstryxError::LocatedError(span_to_location(*k), AstryxErrorKind::Unexpected)
                    })?,
                )?;
            }

            match eval_expression(state, &*f.ident, None)? {
                Object::BuiltinFunction(builtin) => builtin(Rc::new(RefCell::new(inner)), input),
                _ => unimplemented!(),
            }
        }
        Expression::Reference(r) => match input {
            Some(n) => match n.borrow().clone() {
                Object::None => Ok(Object::None),
                Object::Map(m) => Ok(m.get(&r.to_string()).unwrap().borrow().clone()),
                _ => unimplemented!(),
            },
            None => state
                .borrow()
                .get(&r.to_string())
                .ok_or(AstryxError::LocatedError(
                    span_to_location(*r),
                    AstryxErrorKind::Unexpected,
                )),
        },
        Expression::Literal(l) => match l {
            parser::Literal::String(s) => Ok(Object::String(s.to_string())),
            parser::Literal::Number(_s, f) => Ok(Object::Number(f.clone())),
        },
        Expression::RelativePath(s) => Ok(Object::Path(s.to_string())),
        Expression::Array(arr) => Ok(Object::Array(
            arr.iter()
                .map(|el| eval_expression(Rc::clone(&state), el, None))
                .collect::<AstryxResult<Vec<Object>>>()?
                .into_iter()
                .map(Node::new)
                .collect(),
        )),
        Expression::GlobPattern(s) => crate::util::glob_files(s, state.borrow().get("$PWD")),
        Expression::Index(l, r) => {
            let lexpr: Object = eval_expression(Rc::clone(&state), l, None)?;

            // create micro state
            // let mut inner = State::new();

            // check if state needs to be built
            // match &lexpr {
            //     Object::None => panic!("encountered none state"),
            //     Object::String(_) => unimplemented!(),
            //     Object::Number(_) => unimplemented!(),
            //     Object::Path(_) => {}
            //     Object::HTMLPage(_) => unimplemented!(),
            //     Object::HTMLElement(_) => unimplemented!(),
            //     Object::File(_) => unimplemented!(),
            //     Object::BuiltinFunction(_) => unimplemented!(),
            //     Object::Array(_) => unimplemented!(),
            //     Object::Map(m) => {}
            // };

            // println!("state: {:?}", state.borrow().local);

            // note: state should not be used, just the builtins / object functions should work
            // this creates a state with invalid variables.
            eval_expression(state, r, Some(Node::new(lexpr)))

            // match &**r {
            //     Expression::FunctionCall(f) => {
            //         for (k, expr) in &f.arguments {
            //             inner.bind(
            //                 &k.to_string(),
            //                 eval_expression(Rc::clone(&state), expr, None)?,
            //             )?;
            //         }

            //         match eval_expression(state, &*f.ident, None)? {
            //             Object::BuiltinFunction(builtin) => {
            //                 builtin(Rc::new(RefCell::new(inner)), Some(Node::new(lexpr)))
            //             }
            //             _ => unimplemented!(),
            //         }
            //     }

            //     Expression::Reference(ident) => {
            //         eval_expression(Rc::new(RefCell::new(inner)), ident, input)
            //     }

            //     // lexpr
            //     //     .get(r.to_string().as_str())
            //     //     .map(|o| o.borrow().clone())
            //     //     .ok_or(AstryxError::LocatedError(
            //     //         r.into(),
            //     //         AstryxErrorKind::Unexpected,
            //     //     )),
            //     Expression::Literal(_) => unimplemented!(),
            //     Expression::Array(_) => unimplemented!(),
            //     Expression::Index(_, _) => unimplemented!(),

            //     // should never work with index notation:
            //     Expression::GlobPattern(_) => unimplemented!(),
            //     Expression::RelativePath(_) => unimplemented!(),
            // }

            // match &lexpr {
            //     Object::Map(ref m) => match **r {
            //         Expression::Reference(r) => m
            //             .get(r.to_string().as_str())
            //             .map(|o| o.borrow().clone())
            //             .ok_or(AstryxError::LocatedError(
            //                 r.into(),
            //                 AstryxErrorKind::Unexpected,
            //             )),

            //         Expression::FunctionCall(_) => unimplemented!(),
            //         Expression::GlobPattern(_) => unimplemented!(),
            //         Expression::RelativePath(_) => unimplemented!(),
            //         Expression::Literal(_) => unimplemented!(),
            //         Expression::Array(_) => unimplemented!(),
            //         Expression::Index(_, _) => unimplemented!(),
            //     },
            //     Object::String(s) => match &**r {
            //         Expression::FunctionCall(f) => {
            //             // println!("string.fn: {:?}", lexpr);
            //             let mut inner = State::new();
            //             inner.bind("$self", lexpr.clone())?;

            //             for (k, expr) in &f.arguments {
            //                 inner.bind(
            //                     &k.to_string(),
            //                     eval_expression(Rc::clone(&state), expr, None)?,
            //                 )?;
            //             }

            //             match eval_expression(state, &*f.ident, None)? {
            //                 Object::BuiltinFunction(builtin) => {
            //                     builtin(Rc::new(RefCell::new(inner)), Some(Node::new(lexpr)))
            //                 }
            //                 _ => unimplemented!(),
            //             }
            //         }
            //         Expression::GlobPattern(_) => unimplemented!(),
            //         Expression::RelativePath(_) => unimplemented!(),
            //         Expression::Reference(_) => unimplemented!(),
            //         Expression::Literal(_) => unimplemented!(),
            //         Expression::Array(_) => unimplemented!(),
            //         Expression::Index(_, _) => unimplemented!(),
            //     },
            //     _ => panic!("{}", lexpr.inspect()),
            // }
        }
    }
}

// fn eval_reference<'a>(name: &Span<'a>, state: Rc<RefCell<State>>) -> InterpreterResult<Object> {
//     state
//         .borrow()
//         .get(&name.fragment().to_string())
//         .ok_or(InterpreterError {
//             kind: InterpreterErrorKind::InvalidReference(name.to_string()),
//             location: Some((*name).into()),
//         })
// }

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
                Ok(eval_expression(Rc::clone(&state), &expr, None)?.to_string())
            }
        })
        .collect::<Result<Vec<String>, AstryxError>>()?
        .into_iter()
        .collect())
}

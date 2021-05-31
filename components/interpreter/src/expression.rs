use std::cell::RefCell;
use std::rc::Rc;

use crate::util::span_to_location;
use error::{AstryxError, AstryxErrorKind, AstryxResult};
use models::{Node, Object, State};
use parser::{Expression, FunctionCallArguments, Span};
use std::path::PathBuf;

pub fn eval_expression(
    state: Rc<RefCell<State>>,
    expr: &Expression,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    match expr {
        Expression::FunctionCall(ref f) => {
            // try to resolve the symbol to a function
            match eval_expression(Rc::clone(&state), &*f.ident, None)? {
                Object::BuiltinFunction(builtin) => {
                    let mut inner = State::new();

                    // add function arguments into scope
                    match &f.arguments {
                        FunctionCallArguments::None => (),
                        FunctionCallArguments::Named(args) => {
                            for (k, expr) in args {
                                inner.bind(
                                    // the symbol side
                                    &k.to_string(),
                                    // evaluate the expression side
                                    eval_expression(Rc::clone(&state), expr, None)?,
                                )?;
                            }
                        }
                        FunctionCallArguments::Unnamed(args) => {
                            // need to get symbols from the Function...
                            todo!();
                            // for (k, expr) in args {
                            //     inner.bind(
                            //         // the symbol side
                            //         &k.to_string(),
                            //         // evaluate the expression side
                            //         eval_expression(Rc::clone(&state), expr, None)?,
                            //     )?;
                            // }
                        }
                    };

                    builtin(Rc::new(RefCell::new(inner)), input)
                }
                _ => unimplemented!(), // throw error
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
                    AstryxErrorKind::UnknownValue(r.fragment().to_string()),
                )),
        },
        Expression::Literal(l) => match l {
            parser::Literal::String(s) => Ok(Object::String(s.to_string())),
            parser::Literal::Number(_s, f) => Ok(Object::Number(f.clone())),
        },
        Expression::RelativePath(s) => {
            let context_file = PathBuf::from(s.extra.to_string());
            let rebased_file = context_file
                .parent()
                .expect("relative path with no context file. this should not happen.")
                .join(s.to_string());

            Ok(Object::Path(
                rebased_file
                    .to_str()
                    .expect("os string to convert")
                    .to_string(),
            ))
        }
        Expression::Array(arr) => Ok(Object::Array(
            arr.iter()
                .map(|el| eval_expression(Rc::clone(&state), el, None))
                .collect::<AstryxResult<Vec<Object>>>()?
                .into_iter()
                .map(Node::new)
                .collect(),
        )),
        Expression::GlobPattern(pattern) => glob(pattern),
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

fn glob(span: &Span) -> AstryxResult<Object> {
    let context_file = PathBuf::from(span.extra.to_string());
    let rebased_file = context_file
        .parent()
        .expect("relative path with no context file. this should not happen.")
        .join(span.to_string());
    let pattern = format!("./{}", rebased_file.to_str().unwrap()); // todo: security checks
    crate::util::glob_files(pattern).map_err(|e| {
        AstryxError::LocatedError(
            span_to_location(*span),
            AstryxErrorKind::FilePatternError(e.to_string()),
        )
    })
}

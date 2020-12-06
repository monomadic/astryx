use crate::{
    models::Object, state::State, InterpreterError, InterpreterErrorKind, InterpreterResult,
};
use html::HTMLElement;
use parser::{Expression, Statement, StringToken};
use rctree::Node;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

pub(crate) fn eval_statement<'a>(
    statement: &Node<Statement<'a>>,
    state: Rc<RefCell<State>>,
) -> InterpreterResult<Node<Object>> {
    // this needs to be Node<Object>
    match statement.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

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
            // program.push(ProgramInstruction::Text(element.clone().open_tag()));

            // state
            //     .borrow()
            //     .push_instruction(ProgramInstruction::Text(element.clone().open_tag()));

            let node = Node::new(Object::HTMLElement(element));

            for child in statement.children() {
                eval_statement(&child, Rc::clone(&state))?;
                // node.append child
            }

            // state
            //     .borrow()
            //     .push_instruction(ProgramInstruction::Text(element.clone().close_tag()));

            Ok(node)
        }
        Statement::Expression(expr) => {
            let return_objects: Vec<Node<Object>> = statement
                .children()
                .map(|child| eval_statement(&child, Rc::clone(&state)))
                .collect::<InterpreterResult<Vec<Node<Object>>>>()?;

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
        Statement::Text(t) => Ok(Node::new(Object::String(eval_interpolation(
            Rc::clone(&state),
            t,
        )?))),
        Statement::Binding(ident, expr) => {
            let obj = eval_expression(Rc::clone(&state), &expr, None)?;
            state.borrow_mut().bind(ident.fragment(), obj.clone())?;
            return Ok(Node::new(Object::None));
        }
        Statement::Comment(_) => Ok(Node::new(Object::None)),
        Statement::ForLoop { ident, expr } => {
            let iter: Object = eval_expression(Rc::clone(&state), &expr, None)?;

            if let Object::Array(array) = iter {
                for index in array {
                    let childstate = state.clone();
                    childstate
                        .borrow_mut()
                        .bind(&ident.to_string(), index.borrow().clone())?;
                    for child in statement.children() {
                        // BUG HERE - CHILDSTATE IS THE SAME
                        // println!("---{:?}", &childstate.borrow().local);
                        let _ = eval_statement(&child, Rc::clone(&childstate))?;
                    }
                }
            } else {
                return Err(InterpreterError {
                    kind: InterpreterErrorKind::UnexpectedToken {
                        expected: String::from("Array"),
                        got: iter.inspect(),
                    },
                    location: Some(ident.into()),
                });
            }

            Ok(Node::new(Object::None)) // FIXME
        }
    }
}

pub fn eval_expression<'a>(
    state: Rc<RefCell<State>>,
    expr: &Expression<'a>,
    input: Option<Node<Object>>,
) -> InterpreterResult<Object> {
    match expr {
        Expression::FunctionCall(ref f) => {
            let mut inner = State::new();
            inner.program = Rc::clone(&state.borrow().program);

            for (k, expr) in &f.arguments {
                inner.bind(
                    &k.to_string(),
                    eval_expression(Rc::clone(&state), expr, None)?,
                )?;
            }

            match eval_expression(state, &*f.ident, None)? {
                Object::BuiltinFunction(builtin) => builtin(Rc::new(RefCell::new(inner)), None),
                _ => unimplemented!(),
            }
        }
        Expression::Reference(r) => state.borrow().require(r),
        Expression::Literal(l) => match l {
            parser::Literal::String(s) => Ok(Object::String(s.to_string())),
            parser::Literal::Number(_s, f) => Ok(Object::Number(f.clone())),
        },
        Expression::RelativePath(_) => unimplemented!(),
        Expression::Array(_) => unimplemented!(),
        Expression::GlobPattern(s) => crate::util::import_files(s),
        Expression::Index(l, r) => {
            let lexpr = eval_expression(Rc::clone(&state), l, None)?;

            // todo: FIX THIS, create micro-scope and use as input

            match &lexpr {
                Object::Map(ref m) => match **r {
                    Expression::Reference(r) => m
                        .get(r.to_string().as_str())
                        .map(|o| o.borrow().clone())
                        .ok_or(InterpreterError {
                            kind: InterpreterErrorKind::UnknownMemberFunction(r.to_string()),
                            location: Some(r.into()),
                        }),

                    Expression::FunctionCall(_) => unimplemented!(),
                    Expression::GlobPattern(_) => unimplemented!(),
                    Expression::RelativePath(_) => unimplemented!(),
                    Expression::Literal(_) => unimplemented!(),
                    Expression::Array(_) => unimplemented!(),
                    Expression::Index(_, _) => unimplemented!(),
                },
                Object::String(s) => match &**r {
                    Expression::FunctionCall(f) => {
                        // println!("string.fn: {:?}", lexpr);
                        let mut inner = State::new();
                        inner.bind("$self", lexpr.clone())?;

                        for (k, expr) in &f.arguments {
                            inner.bind(
                                &k.to_string(),
                                eval_expression(Rc::clone(&state), expr, None)?,
                            )?;
                        }

                        match eval_expression(state, &*f.ident, None)? {
                            Object::BuiltinFunction(builtin) => {
                                builtin(Rc::new(RefCell::new(inner)), Some(Node::new(lexpr)))
                            }
                            _ => unimplemented!(),
                        }
                    }
                    Expression::GlobPattern(_) => unimplemented!(),
                    Expression::RelativePath(_) => unimplemented!(),
                    Expression::Reference(_) => unimplemented!(),
                    Expression::Literal(_) => unimplemented!(),
                    Expression::Array(_) => unimplemented!(),
                    Expression::Index(_, _) => unimplemented!(),
                },
                _ => panic!("{}", lexpr.inspect()),
            }
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
fn eval_interpolation<'a>(
    state: Rc<RefCell<State>>,
    components: Vec<StringToken<'a>>,
) -> InterpreterResult<String> {
    Ok(components
        .into_iter()
        .map(|st| match st {
            StringToken::Text(span) => Ok(span.to_string()),
            // StringToken::Expression(expr) => self.eval(&expr).map(|e| e.into()),
            StringToken::Expression(expr) => {
                Ok(eval_expression(Rc::clone(&state), &expr, None)?.to_string())
            }
        })
        .collect::<Result<Vec<String>, InterpreterError>>()?
        .into_iter()
        .collect())
}

use crate::{
    models::Object, state::State, InterpreterError, InterpreterErrorKind, InterpreterResult,
};
use html::HTMLElement;
use parser::{FunctionCall, Span, Statement};
use program::ProgramInstruction;
use rctree::Node;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

pub(crate) fn eval_statement<'a>(
    statement: &Node<Statement<'a>>,
    state: Rc<RefCell<State<'a>>>,
    program: &mut Vec<ProgramInstruction>, // could this be passed around in a RefCell inside state?
) -> InterpreterResult<()> {
    match statement.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            for (ident, expr) in e.attributes {
                attributes.insert(
                    ident.fragment().to_string(),
                    state.borrow().eval_expression(&expr)?.into(),
                );
            }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            // todo, these really should be html nodes, so that we can optimise them all later...
            // examples:
            // - removing empty or unneeded classes/styles/ids/empty elements (optional)
            // - link rewriters
            // - searching for specific elements
            // - injection? (might need tree for this though...?)
            program.push(ProgramInstruction::Text(element.clone().open_tag()));

            for child in statement.children() {
                eval_statement(&child, state.clone(), program)?;
            }

            program.push(ProgramInstruction::Text(element.clone().close_tag()));
        }
        Statement::Expression(expr) => {
            state.borrow().eval_expression(&expr)?;
        }
        Statement::Text(t) => {
            program.push(ProgramInstruction::Text(state.borrow().interpolate(t)?));
        }
        Statement::Binding(ident, expr) => {
            let obj = state.borrow().eval_expression(&expr)?;
            state.borrow_mut().bind(ident.fragment(), obj.clone())?;
        }
        Statement::Comment(_) => {}
        Statement::ForLoop { ident, expr } => {
            let iter: Object = state.borrow().eval_expression(&expr)?;

            if let Object::Array(array) = iter {
                for index in array {
                    let childstate = state.clone();
                    childstate.borrow_mut().bind(&ident.to_string(), index)?;
                    for child in statement.children() {
                        let _ = eval_statement(&child, Rc::clone(&childstate), program)?;
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
        }
    }

    Ok(())
}

// pub(crate) fn eval_expression<'a>(
//     state: Rc<RefCell<State<'a>>>,
//     expr: &Expression<'a>,
// ) -> InterpreterResult<Object<'a>> {
//     match expr {
//         // Expression::FunctionCall(f) => state.eval_function(&f)?,
//         Expression::FunctionCall(f) => eval_function(Rc::clone(&state), &f),
//         Expression::Reference(r) => eval_reference(&r, Rc::clone(&state)),
//         Expression::Literal(l) => match l {
//             Literal::String(s) => Ok(Object::String(s.fragment().to_string())),
//             Literal::Float(_, _) => unimplemented!(),
//         },
//         Expression::RelativePath(s) => import_file(s),
//         Expression::Array(_) => unimplemented!(),
//         Expression::GlobPattern(s) => import_files(s),
//         Expression::Index(i, e) => unimplemented!(),
//     }
// }

fn eval_function<'a>(
    state: Rc<RefCell<State<'a>>>,
    func: &FunctionCall<'a>,
) -> InterpreterResult<Object> {
    let ident_ref = *(func.clone()).ident;
    let function = state.borrow().eval_expression(&ident_ref)?;

    match function {
        // Object::FunctionLiteral {
        //     params: _,
        //     statements: _,
        // } => {
        //     // extend state scope into function
        //     let _new_env = Rc::new(RefCell::new(State::extend(state)));

        //     // insert args into new scope
        //     // let arguments = eval_expressions(args, env)?;

        //     // apply_function(&function, &vec![])
        //     unimplemented!()
        // }
        Object::BuiltinFunction(builtin) => {
            // println!("ARGS {:?}", func.arguments);
            builtin(state.borrow().eval_function_arguments(&func.arguments)?)
        }
        // _ => Err(InterpreterError::ReferenceIsNotAFunction),
        Object::String(s) => {
            println!("sss{:?}", s);
            unimplemented!()
        }
        Object::Array(_) => unimplemented!(),
        Object::Map(_) => unimplemented!(),
    }
}

fn apply_function<'a>(func: &Object, arguments: &Vec<Object>) -> InterpreterResult<Object> {
    // assert_argument_count(params.len(), &arguments)?;
    // let new_env = extend_function_env(params, arguments, env);

    // for statement in func
    // let evaluated = eval_block_statement(&body, new_env)?;
    // unwrap_return_value(evaluated)
    unimplemented!()
}

fn eval_reference<'a>(name: &Span<'a>, state: Rc<RefCell<State<'a>>>) -> InterpreterResult<Object> {
    state
        .borrow()
        .get(&name.fragment().to_string())
        .ok_or(InterpreterError {
            kind: InterpreterErrorKind::InvalidReference(name.to_string()),
            location: Some((*name).into()),
        })
}

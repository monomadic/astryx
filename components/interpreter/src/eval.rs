use crate::{models::Object, state::State, InterpreterError, InterpreterResult};
use html::{HTMLElement, HTMLNode};
use parser::{Expression, FunctionCall, Literal, Span, Statement};
use program::ProgramInstruction;
use rctree::Node;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

pub(crate) fn eval_statement<'a>(
    statement: &Node<Statement<'a>>,
    state: Rc<RefCell<State<'a>>>,
    program: &mut Vec<ProgramInstruction>,
) -> InterpreterResult<()> {
    match statement.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            for (ident, expr) in e.attributes {
                attributes.insert(
                    ident.fragment().to_string(),
                    eval_expression(Rc::clone(&state), &expr)?.into(),
                );
            }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            program.push(ProgramInstruction::Text(element.clone().open_tag()));

            println!("pr: {:?}", program);

            // state.borrow_mut().write(&ot)?;

            for child in statement.children() {
                eval_statement(&child, state.clone(), program);
            }

            program.push(ProgramInstruction::Text(element.clone().close_tag()));
            // state.borrow_mut().write(&ct)?;
        }
        Statement::Expression(expr) => {
            eval_expression(state, &expr)?;
        }
        Statement::Text(t) => {
            let text = state.borrow_mut().interpolate(t)?;
            state.borrow_mut().write(&text)?;

            let element_node = ProgramInstruction::Text(text);
            program.push(element_node);
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
        Statement::Comment(_) => {}
        Statement::ForLoop { ident, expr } => {
            let iter: Object = eval_expression(Rc::clone(&state), &expr)?;
            // println!("iter {}", iter.inspect());

            if let Object::Array(array) = iter {
                for index in array {
                    let childstate = state.clone();
                    childstate.borrow_mut().bind(&ident.to_string(), index)?;
                    for child in statement.children() {
                        let _ = eval_statement(&child, Rc::clone(&childstate), program);
                    }
                }
            } else {
                println!("not array {:?}", iter.inspect());
                return Err(InterpreterError::UnexpectedToken {
                    expected: String::from("Array"),
                    got: iter.inspect(),
                });
            }
        }
    }

    Ok(())
}

pub(crate) fn eval_expression<'a>(
    state: Rc<RefCell<State<'a>>>,
    expr: &Expression<'a>,
) -> InterpreterResult<Object<'a>> {
    match expr {
        // Expression::FunctionCall(f) => state.eval_function(&f)?,
        Expression::FunctionCall(f) => eval_function(Rc::clone(&state), &f),
        Expression::Reference(r) => eval_reference(&r, Rc::clone(&state)),
        Expression::Literal(l) => match l {
            Literal::String(s) => Ok(Object::String(s.fragment().to_string())),
            Literal::Float(_, _) => unimplemented!(),
        },
        Expression::RelativePath(s) => import_file(s),
        Expression::Array(_) => unimplemented!(),
        Expression::GlobPattern(s) => import_files(s),
    }
}

fn import_file<'a>(s: &Span<'a>) -> InterpreterResult<Object<'a>> {
    std::fs::read_to_string(s.fragment().to_string())
        .map(Object::String)
        .map_err(|_| InterpreterError::IOError)
}

fn import_files<'a>(s: &Span<'a>) -> InterpreterResult<Object<'a>> {
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs = glob::glob_with(&s.to_string(), options).map_err(|_| InterpreterError::IOError)?;

    for file in globs {
        // TODO wrap unwrap in error
        let path = file.expect("file to unwrap");
        let filepath: String = path.as_os_str().to_str().unwrap().into();
        let file_content = std::fs::read_to_string(filepath).unwrap();

        files.push(Object::String(file_content));
    }

    Ok(Object::Array(files))
}

fn eval_function<'a>(
    state: Rc<RefCell<State<'a>>>,
    func: &FunctionCall<'a>,
) -> InterpreterResult<Object<'a>> {
    let ident_ref = *(func.clone()).ident;
    let function = eval_expression(Rc::clone(&state), &ident_ref)?;

    match function {
        Object::FunctionLiteral {
            params: _,
            statements: _,
        } => {
            // extend state scope into function
            let _new_env = Rc::new(RefCell::new(State::extend(state)));

            // insert args into new scope
            // let arguments = eval_expressions(args, env)?;

            // apply_function(&function, &vec![])
            unimplemented!()
        }
        Object::BuiltinFunction(builtin) => {
            // println!("ARGS {:?}", func.arguments);
            builtin(eval_function_arguments(Rc::clone(&state), &func.arguments)?)
        }
        // _ => Err(InterpreterError::ReferenceIsNotAFunction),
        Object::String(s) => {
            println!("sss{:?}", s);
            unimplemented!()
        }
        Object::Array(_) => unimplemented!(),
    }
}

fn eval_function_arguments<'a>(
    state: Rc<RefCell<State<'a>>>,
    args: &Vec<(Span<'a>, Expression<'a>)>,
) -> InterpreterResult<Vec<Object<'a>>> {
    args.into_iter()
        .map(|(_ident, expr)| eval_expression(Rc::clone(&state), expr))
        // .collect::<Vec<InterpreterResult<Object<'a>>>>()
        .collect::<Result<Vec<Object<'a>>, InterpreterError>>()
    // .collect()
}

fn apply_function<'a>(func: &Object, arguments: &Vec<Object>) -> InterpreterResult<Object<'a>> {
    // assert_argument_count(params.len(), &arguments)?;
    // let new_env = extend_function_env(params, arguments, env);

    // for statement in func
    // let evaluated = eval_block_statement(&body, new_env)?;
    // unwrap_return_value(evaluated)
    unimplemented!()
}

fn eval_reference<'a>(
    name: &Span<'a>,
    state: Rc<RefCell<State<'a>>>,
) -> InterpreterResult<Object<'a>> {
    state
        .borrow()
        .get(&name.fragment().to_string())
        .ok_or(InterpreterError::InvalidReference(name.to_string()))
}

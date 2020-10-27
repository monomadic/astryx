use crate::{models::Object, state::State, InterpreterError, InterpreterResult};
use html::HTMLElement;
use parser::{Expression, FunctionCall, Literal, Span, Statement};
use rctree::Node;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

pub(crate) fn eval_statement<'a>(
    node: &Node<Statement<'a>>,
    state: Rc<RefCell<State<'a>>>,
) -> InterpreterResult<()> {
    match node.borrow().clone() {
        Statement::Element(e) => {
            let mut attributes: HashMap<String, String> = HashMap::new();

            for (ident, expr) in e.attributes {
                attributes.insert(
                    ident.fragment().to_string(),
                    eval_expression(Rc::clone(&state), &expr)?.into(),
                );
            }

            let element = HTMLElement::new(e.ident.fragment(), attributes).expect("valid html");

            state.borrow_mut().write(&element.open_tag())?;

            for child in node.children() {
                let _ = eval_statement(&child, state.clone());
            }

            state.borrow_mut().write(&element.close_tag())?;
        }
        Statement::Expression(expr) => {
            eval_expression(state, &expr)?;
        }
        Statement::Text(t) => {
            let text = state.borrow_mut().interpolate(t)?;
            state.borrow_mut().write(&text)?;
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
    }

    Ok(())
}

fn eval_expression<'a>(
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
    }
}

fn eval_function<'a>(
    state: Rc<RefCell<State<'a>>>,
    func: &FunctionCall<'a>,
) -> InterpreterResult<Object<'a>> {
    let ident_ref = *(func.clone()).ident;
    let function = eval_expression(Rc::clone(&state), &ident_ref)?;

    match function {
        Object::FunctionLiteral { params, statements } => {
            // extend state scope into function
            let new_env = Rc::new(RefCell::new(State::extend(state)));

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

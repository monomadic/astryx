use crate::{models::Object, InterpreterResult, State};
use parser::Span;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn import(state: Rc<RefCell<State>>) -> Rc<RefCell<State>> {
    let _ = state.borrow_mut().bind("log", Object::BuiltinFunction(log));

    let _ = state
        .borrow_mut()
        .bind("frontmatter", Object::BuiltinFunction(frontmatter));

    let _ = state
        .borrow_mut()
        .bind("inspect", Object::BuiltinFunction(inspect));

    let _ = state
        .borrow_mut()
        .bind("markdown", Object::BuiltinFunction(markdown));

    state
}

pub(crate) fn log(state: Rc<RefCell<State>>) -> InterpreterResult<Object> {
    let args = state
        .borrow()
        .local
        .iter()
        .map(|(_k, v)| format!("{}", v.to_string()))
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", args);
    Ok(Object::String(String::new())) // todo: return ()
}

/// returns a debug representation of an object as a string
pub(crate) fn inspect(state: Rc<RefCell<State>>) -> InterpreterResult<Object> {
    let args = state
        .borrow()
        .local
        .iter()
        .map(|(_k, v)| format!("{:?}", v.to_string()))
        .collect::<Vec<String>>()
        .join(", ");

    Ok(Object::String(args)) // todo: return ()
}

pub(crate) fn markdown<'a>(
    state: Rc<RefCell<State>>,
) -> InterpreterResult<Object> {
    let doc = state.borrow().require(&Span::new("$self"))?;
    let result = markdown::parse(&doc.to_string()).unwrap();
    Ok(Object::String(result))
}

pub(crate) fn frontmatter<'a>(
    state: Rc<RefCell<State>>,
) -> InterpreterResult<Object> {
    Ok(state.borrow().require(&Span::new("$self"))?)
}

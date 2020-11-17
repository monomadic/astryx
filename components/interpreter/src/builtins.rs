use crate::{models::Object, InterpreterResult, State};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn log(args: Vec<Object>) -> InterpreterResult<Object> {
    let display = args
        .iter()
        .map(|o| format!("{}", o.to_string()))
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", display);
    Ok(Object::String(display)) // todo: return ()
}

pub(crate) fn debug(args: Vec<Object>) -> InterpreterResult<Object> {
    let display = args
        .iter()
        .map(|o| format!("{}", o.to_string()))
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", display);
    Ok(Object::String(display)) // todo: return ()
}

pub(crate) fn import<'a>(state: Rc<RefCell<State<'a>>>) -> Rc<RefCell<State<'a>>> {
    // let mut inner = state.borrow_mut();
    let _ = state.borrow_mut().bind("log", Object::BuiltinFunction(log));

    let _ = state
        .borrow_mut()
        .bind("debug", Object::BuiltinFunction(debug));

    let _ = state
        .borrow_mut()
        .bind("markdown", Object::BuiltinFunction(markdown));

    state
}

pub(crate) fn markdown<'a>(args: Vec<Object>) -> InterpreterResult<Object> {
    Ok(Object::String("markdownnnnn".into()))
}

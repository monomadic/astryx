use crate::{models::Object, InterpreterResult, State};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn print(args: Vec<Object>) -> InterpreterResult<Object> {
    let display = args
        .iter()
        .map(|o| format!("{}", o))
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", display);
    Ok(Object::String(display)) // todo: return ()
}

pub(crate) fn debug(args: Vec<Object>) -> InterpreterResult<Object> {
    let display = args
        .iter()
        .map(|o| format!("{:?}", o))
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", display);
    Ok(Object::String(display)) // todo: return ()
}

pub(crate) fn import<'a>(state: Rc<RefCell<State<'a>>>) -> Rc<RefCell<State<'a>>> {
    // let mut inner = state.borrow_mut();
    let _ = state
        .borrow_mut()
        .bind("print", Object::BuiltinFunction(print));

    state
}

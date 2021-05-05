use error::{AstryxError, AstryxResult};
use models::{object::Object, state::State};
use rctree::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn import(state: Rc<RefCell<State>>) -> Rc<RefCell<State>> {
    let _ = state.borrow_mut().bind("log", Object::BuiltinFunction(log));

    let _ = state
        .borrow_mut()
        .bind("frontmatter", Object::BuiltinFunction(parse_frontmatter));

    let _ = state
        .borrow_mut()
        .bind("markdown", Object::BuiltinFunction(markdown));

    let _ = state
        .borrow_mut()
        .bind("asset", Object::BuiltinFunction(asset));

    state
}

pub(crate) fn log(state: Rc<RefCell<State>>, input: Option<Node<Object>>) -> AstryxResult<Object> {
    match input {
        Some(input) => {
            println!("{:?}", input.borrow().to_string());
            Ok(input.borrow().clone())
        }
        None => {
            println!(
                "{:?}",
                state
                    .borrow()
                    .local
                    .iter()
                    .map(|(_k, v)| format!("{}", v.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            Ok(Object::None)
        }
    }
}

// /// returns a debug representation of an object as a string
// pub(crate) fn inspect_all(state: Rc<RefCell<State>>) -> AstryxResult<Object> {
//     let args = state
//         .borrow()
//         .local
//         .iter()
//         .map(|(_k, v)| format!("{:?}", v.to_string()))
//         .collect::<Vec<String>>()
//         .join(", ");
//
//     Ok(Object::String(args)) // todo: return ()
// }

pub(crate) fn markdown<'a>(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    let path = match input {
        Some(input) => input.borrow().clone(),
        None => state
            .borrow()
            .get("path")
            .ok_or(AstryxError::Generic("variable path not found".into()))?, // todo: better error class
    };

    let content = read(state, Some(Node::new(path)))?.to_string();

    Ok(Object::String(markdown::parse(&content).unwrap()))
}

pub(crate) fn parse_frontmatter<'a>(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    let path = match input {
        Some(input) => input.borrow().clone(),
        None => state
            .borrow()
            .get("path")
            .ok_or(AstryxError::Generic("variable path not found".into()))?,
    };

    let content = read(state, Some(Node::new(path)))?.to_string();

    let (yaml, _document) = frontmatter::parse(&content).unwrap();

    if let Some(yaml) = yaml {
        Ok(yaml.into())
    } else {
        Ok(Object::Map(HashMap::new()))
    }
}

pub(crate) fn asset<'a>(
    state: Rc<RefCell<State>>,
    _input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    let path = state
        .borrow()
        .get("path")
        .ok_or(AstryxError::Generic("variable path not found".into()))?;

    // Ok(input.unwrap().borrow().clone())

    Ok(Object::File(path.to_string()))
}

// /// takes an object and writes to a file
// pub(crate) fn write<'a>(
//     state: Rc<RefCell<State>>,
//     _input: Option<Node<Object>>,
// ) -> AstryxResult<Object> {
//     let path = state
//         .borrow()
//         .get("path")
//         .ok_or(AstryxError::Generic(format!("no $self")))?;

//     // state
//     //     .borrow()
//     //     .push_instruction(ProgramInstruction::SetPath(path.to_string()));

//     Ok(Object::None)
// }

/// takes an path and writes to an object
pub(crate) fn read<'a>(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    let path = match input {
        Some(input) => input.borrow().clone(),
        None => state
            .borrow()
            .get("path")
            .ok_or(AstryxError::Generic("variable path not found".into()))?,
    };

    match path {
        Object::String(s) | Object::Path(s) => std::fs::read_to_string(s)
            .map(Object::String)
            .map_err(|_| AstryxError::Generic("<can't read file>".into())),
        _ => unimplemented!(), // return error
    }
}

// fn assert_argument_count(count: usize) -> EvalResult {}

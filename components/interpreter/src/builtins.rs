use error::{AstryxError, AstryxResult};
use models::{object::Object, state::State, BuiltinFunction};
use rctree::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn import(state: Rc<RefCell<State>>) -> Rc<RefCell<State>> {
    let _ = state.borrow_mut().bind(
        "print",
        Object::BuiltinFunction(BuiltinFunction {
            args: vec!["obj".to_string()],
            closure: print,
        }),
    );

    let _ = state.borrow_mut().bind(
        "copy",
        Object::BuiltinFunction(BuiltinFunction {
            args: vec!["file".to_string()],
            closure: copy_file,
        }),
    );

    // let _ = state
    //     .borrow_mut()
    //     .bind("frontmatter", Object::BuiltinFunction(parse_frontmatter));
    //
    // let _ = state
    //     .borrow_mut()
    //     .bind("markdown", Object::BuiltinFunction(markdown));
    //
    // let _ = state
    //     .borrow_mut()
    //     .bind("asset", Object::BuiltinFunction(asset));
    //
    // let _ = state
    //     .borrow_mut()
    //     .bind("require", Object::BuiltinFunction(require));

    state
}

/// an is_empty() that fails on empty... probably rename this?
pub(crate) fn require(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    let iter = match input.clone() {
        Some(input) => input.borrow().clone(),
        None => state
            .borrow()
            .get("iter")
            .ok_or(AstryxError::Generic("variable path not found".into()))?, // todo: better error class
    };

    match &iter {
        Object::Array(arr) => {
            if arr.is_empty() {
                Ok(iter)
            } else {
                Err(AstryxError::Generic("object not empty".into()))
            }
        }
        _ => Err(AstryxError::Generic("object not an array".into())),
    }
}

pub(crate) fn copy_file(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
    let path = match input.clone() {
        Some(input) => input.borrow().clone(),
        None => state
            .borrow()
            .get("path")
            .ok_or(AstryxError::Generic("variable path not found".into()))?, // todo: better error class
    };

    // fixme: path should be relative to $PWD variable

    // panic!("file {:?}", input);

    Ok(Object::File(path.to_string()))
}

pub(crate) fn print(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> AstryxResult<Object> {
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
    let (_yaml, document) = frontmatter::parse(&content).unwrap();

    Ok(Object::String(markdown::parse(&document).unwrap()))
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

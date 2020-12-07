use crate::{models::Object, InterpreterResult, State};
use frontmatter::Yaml;
use parser::Span;
use program::ProgramInstruction;
use rctree::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn import(state: Rc<RefCell<State>>) -> Rc<RefCell<State>> {
    let _ = state.borrow_mut().bind("log", Object::BuiltinFunction(log));

    let _ = state
        .borrow_mut()
        .bind("frontmatter", Object::BuiltinFunction(parse_frontmatter));

    // let _ = state
    //     .borrow_mut()
    //     .bind("locals", Object::BuiltinFunction(inspect_all));

    // let _ = state
    //     .borrow_mut()
    //     .bind("markdown", Object::BuiltinFunction(markdown));

    let _ = state
        .borrow_mut()
        .bind("page", Object::BuiltinFunction(page));

    let _ = state
        .borrow_mut()
        .bind("write", Object::BuiltinFunction(write));

    state
}

pub(crate) fn log(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> InterpreterResult<Object> {
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

/// returns a debug representation of an object as a string
pub(crate) fn inspect_all(state: Rc<RefCell<State>>) -> InterpreterResult<Object> {
    let args = state
        .borrow()
        .local
        .iter()
        .map(|(_k, v)| format!("{:?}", v.to_string()))
        .collect::<Vec<String>>()
        .join(", ");

    Ok(Object::String(args)) // todo: return ()
}

pub(crate) fn markdown<'a>(state: Rc<RefCell<State>>) -> InterpreterResult<Object> {
    let doc = state.borrow().require(&Span::new("$self"))?;
    let result = markdown::parse(&doc.to_string()).unwrap();
    Ok(Object::String(result))
}

pub(crate) fn parse_frontmatter<'a>(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> InterpreterResult<Object> {
    let doc = state.borrow().require(&Span::new("$self"))?;

    let (yaml, _document) = match doc {
        Object::String(s) => frontmatter::parse(&s).unwrap(),
        _ => {
            unimplemented!();
        }
    };

    if let Some(yaml) = yaml {
        Ok(yaml.into())
    } else {
        Ok(Object::Map(HashMap::new()))
    }
}

impl From<Yaml> for Object {
    fn from(yaml: Yaml) -> Object {
        match yaml {
            Yaml::Real(_) => unimplemented!(),
            Yaml::Integer(_) => unimplemented!(),
            Yaml::String(s) => Object::String(s),
            Yaml::Boolean(_) => unimplemented!(),
            Yaml::Array(_) => unimplemented!(),
            Yaml::Hash(lhm) => {
                let mut h = HashMap::new();

                for (k, v) in lhm {
                    h.insert(k.into_string().unwrap(), Node::new(v.into()));
                }

                Object::Map(h)
            }
            Yaml::Alias(_) => unimplemented!(),
            Yaml::Null => unimplemented!(),
            Yaml::BadValue => unimplemented!(),
        }
    }
}

pub(crate) fn page<'a>(
    state: Rc<RefCell<State>>,
    input: Option<Node<Object>>,
) -> InterpreterResult<Object> {
    println!("page: {:?}", input);
    // let path = state.borrow().require(&Span::new("path"))?;

    // state
    //     .borrow()
    //     .push_instruction(ProgramInstruction::SetPath(path.to_string()));

    Ok(Object::String("".into()))
}

/// takes an object and writes to a file
pub(crate) fn write<'a>(
    state: Rc<RefCell<State>>,
    _input: Option<Node<Object>>,
) -> InterpreterResult<Object> {
    let path = state.borrow().require(&Span::new("path"))?;

    state
        .borrow()
        .push_instruction(ProgramInstruction::SetPath(path.to_string()));

    Ok(Object::None)
}

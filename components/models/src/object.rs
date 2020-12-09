use crate::state::State;
use error::AstryxResult;
use html::HTMLElement;
use rctree::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yaml_rust::Yaml;

pub type BuiltinFunction = fn(Rc<RefCell<State>>, Option<Node<Object>>) -> AstryxResult<Object>;

#[derive(Clone, Debug)]
pub enum Object {
    None,
    String(String),
    Number(f64),
    HTMLElement(HTMLElement),
    // FunctionLiteral {
    //     params: Vec<String>,4
    //     statements: Vec<Statement<'a>>,
    // },
    BuiltinFunction(BuiltinFunction),
    Array(Vec<Node<Object>>),
    Map(HashMap<String, Node<Object>>),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::String(s) => format!("{:?}", s),
            // Object::FunctionLiteral { params, statements } => {
            //     format!("{:?}{:?}", params, statements)
            // }
            Object::BuiltinFunction(f) => format!("builtin_{:?}()", f),
            Object::Array(v) => format!(
                "[{}]",
                v.iter()
                    .map(|node| Object::inspect(&node.borrow().clone()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Object::Map(_) => unimplemented!(),
            Object::None => format!("(None)"),
            Object::Number(f) => f.to_string(),
            Object::HTMLElement(e) => format!("{}{}", e.open_tag(), e.close_tag()),
        }
    }
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::BuiltinFunction(_) => format!("__BuiltinFunction"),
            Object::String(s) => s.clone(),
            // Object::FunctionLiteral { params, statements } => format!("__FunctionLiteral"),
            Object::Array(arr) => format!("{:?}", arr),
            Object::Map(_) => unimplemented!(),
            Object::None => format!("(None)"),
            Object::Number(n) => format!("{}", n),
            Object::HTMLElement(_) => unimplemented!(),
        }
    }
}

impl Into<String> for Object {
    fn into(self) -> String {
        match self {
            Object::String(s) => s,
            // Object::FunctionLiteral { params, statements } => format!("({:?})", params),
            Object::BuiltinFunction(_) => unimplemented!(),
            Object::Array(_) => unimplemented!(),
            Object::Map(_) => unimplemented!(),
            Object::None => unimplemented!(),
            Object::Number(_) => unimplemented!(),
            Object::HTMLElement(_) => unimplemented!(),
        }
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

use crate::InterpreterResult;
use html::HTMLElement;
use parser::Statement;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AstryxNode {
    HTMLElement(HTMLElement),
    Root,
}

pub type BuiltinFunction = fn(Vec<Object>) -> InterpreterResult<Object>;

#[derive(Clone)]
pub enum Object<'a> {
    String(String),
    FunctionLiteral {
        params: Vec<String>,
        statements: Vec<Statement<'a>>,
    },
    BuiltinFunction(BuiltinFunction),
    Array(Vec<Object<'a>>),
    Map(HashMap<String, Object<'a>>),
}

impl Object<'_> {
    pub fn inspect(&self) -> String {
        match self {
            Object::String(s) => format!("{:?}", s),
            Object::FunctionLiteral { params, statements } => {
                format!("{:?}{:?}", params, statements)
            }
            Object::BuiltinFunction(_) => unimplemented!(),
            Object::Array(v) => format!(
                "[{}]",
                v.iter()
                    .map(Object::inspect)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Object::Map(_) => unimplemented!(),
        }
    }
}

// impl std::fmt::Debug for Object<'_> {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         match self {
//             Object::BuiltinFunction(_) => write!(fmt, "__builtin"),
//             _ => write!(fmt, "{:?}", self), // FIX
//         }

//         // write!(fmt, "debug")
//     }
// }

// impl std::fmt::Display for Object<'_> {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         match self {
//             Object::BuiltinFunction(_) => write!(fmt, "__builtin"),
//             // _ => write!(fmt, "{}", self.to_string()), // FIX
//             _ => write!(fmt, "__unknown"),
//         }
//         // write!(fmt, "display")
//         // write!(fmt, "{:?}", self.to_string())
//     }
// }

impl ToString for Object<'_> {
    fn to_string(&self) -> String {
        match self {
            Object::BuiltinFunction(_) => format!("__BuiltinFunction"),
            Object::String(s) => s.into(),
            Object::FunctionLiteral { params, statements } => format!("__FunctionLiteral"),
            Object::Array(_) => unimplemented!(),
            Object::Map(_) => unimplemented!(),
        }
    }
}

impl Into<String> for Object<'_> {
    fn into(self) -> String {
        match self {
            Object::String(s) => s,
            Object::FunctionLiteral { params, statements } => format!("({:?})", params),
            Object::BuiltinFunction(_) => unimplemented!(),
            Object::Array(_) => unimplemented!(),
            Object::Map(_) => unimplemented!(),
        }
    }
}

use crate::InterpreterResult;
use html::HTMLElement;
use parser::Statement;

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
        }
        // write!(fmt, "display")
        // write!(fmt, "{:?}", self.to_string())
        // format!("writer")
    }
}

impl Into<String> for Object<'_> {
    fn into(self) -> String {
        match self {
            Object::String(s) => s,
            Object::FunctionLiteral { params, statements } => format!("({:?})", params),
            Object::BuiltinFunction(_) => unimplemented!(),
        }
    }
}

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

impl std::fmt::Debug for Object<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            // _ => f.debug_map().finish(),
            _ => f.write_str("0"), // FIX
        }
    }
}

impl std::fmt::Display for Object<'_> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        todo!()
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

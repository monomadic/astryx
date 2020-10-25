use html::HTMLElement;
use parser::Statement;

#[derive(Debug)]
pub enum AstryxNode {
    HTMLElement(HTMLElement),
    Root,
}

#[derive(Debug, Clone)]
pub enum Object<'a> {
    String(String),
    FunctionLiteral {
        params: Vec<String>,
        statements: Vec<Statement<'a>>,
    },
}

impl Into<String> for Object<'_> {
    fn into(self) -> String {
        match self {
            Object::String(s) => s,
            Object::FunctionLiteral { params, statements } => format!("({:?})", params),
        }
    }
}

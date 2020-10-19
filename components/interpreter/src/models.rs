use html::HTMLElement;

#[derive(Debug)]
pub enum AstryxNode {
    HTMLElement(HTMLElement),
    Root,
}

#[derive(Debug)]
pub enum Value {
    String(String),
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::String(s) => s,
        }
    }
}

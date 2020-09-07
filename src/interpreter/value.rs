use super::Document;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Value {
    String(String),
    Document(Document),
    Documents(Vec<Document>),
    // Array(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Document(doc) => write!(f, "{}", doc.body),
            Value::String(s) => write!(f, "{}", s),
            Value::Documents(d) => write!(f, "{:?}", d),
            // Value::Array(a) => write!(f, "{:?}", a),
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => s,
            _ => unimplemented!(),
        }
    }
}

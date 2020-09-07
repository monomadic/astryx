use super::Document;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Value {
    // Array(Vec<Value>),
    Document(Document),
    Documents(Vec<Document>),
    Path(String),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Value::Array(a) => write!(f, "{:?}", a),
            Value::Document(doc) => write!(f, "{}", doc.body),
            Value::Documents(d) => write!(f, "{:?}", d),
            Value::Path(p) => write!(f, "{:?}", p),
            Value::String(s) => write!(f, "{}", s),
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

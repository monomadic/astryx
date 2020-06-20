

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub ident: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    Symbol(String),
    Assignment(Value),
}

#[derive(Debug, Clone)]
pub enum Value {
    RelativePath(String),
    QuotedString(String),
}

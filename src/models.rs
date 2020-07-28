use crate::variable::Variable;

#[derive(Debug, Clone)]
pub enum Token {
    ForLoop(ForLoop),
    Element(Element),
    Text(String),
    CodeBlock(CodeBlock),
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub ident: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub index: String,
    pub iterable: String,
    pub children: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub ident: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Token>,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    Symbol(String),
    Decorator(Decorator),
    Class(String),
    NamedAttribute { ident: String, variable: Variable },
}

#[derive(Debug, Clone)]
pub struct Decorator {
    pub ident: String,
    // value: ?
}

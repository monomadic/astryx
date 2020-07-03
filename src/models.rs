use std::{path::PathBuf, collections::HashMap};

#[derive(Debug, Clone)]
pub enum Node {
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
    pub reference: String,
    pub iterable: Variable, // todo: Vec<Metadata>
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub ident: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    Symbol(String),
    Assignment { ident: String, variable: Variable },
}

#[derive(Debug, Clone)]
pub enum Variable {
    RelativePath(String),
    QuotedString(String),
    Reference(String),
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::RelativePath(s) | Variable::QuotedString(s) | Variable::Reference(s) => {
                f.write_str(s)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    path: PathBuf,
    filename: String,
    // created_at: Date
    variables: HashMap<String, String>,
    body: String
}

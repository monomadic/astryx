use std::{path::PathBuf, collections::HashMap};
use crate::error::ParseResult;

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
    pub index: String,
    pub iterable: String,
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

impl Variable {
    pub fn get_required_path(&self, k: &str) -> ParseResult<PathBuf> {
        panic!("");
    }
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
    pub filename: String,
    // created_at: Date
    pub variables: HashMap<String, String>,
    pub body: String
}

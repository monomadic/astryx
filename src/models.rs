use std::{path::PathBuf};
use crate::error::AstryxResult;

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
    NamedAttribute { ident: String, variable: Variable },
}

#[derive(Debug, Clone)]
pub enum Variable {
    RelativePath(String),
    QuotedString(String),
    Reference(String),
    TemplateFile(TemplateFile),
}

// caution: does not resolve references.
impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::RelativePath(s) | Variable::QuotedString(s) | Variable::Reference(s) => {
                f.write_str(s)
            }
            Variable::TemplateFile(t) => {
                f.write_str(&t.body)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateFile {
    // created_at: Date
    pub body: String,
    // pub filename: String,
    // pub variables: HashMap<String, String>,
    pub metadata: Option<yaml_rust::Yaml>,
}

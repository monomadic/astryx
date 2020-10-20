use crate::Span;
use std::fmt::Display;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token {
//     Blank,
//     Letter,
// }

// #[derive(Debug, Clone)]
// pub enum Expression<'a> {
// }

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    Binding(Span<'a>, Expression<'a>),
    Element(Element<'a>),
    Text(Vec<StringToken<'a>>), // todo: replace with interpolatedstring
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    FunctionCall(FunctionCall<'a>),
    Reference(Variable<'a>),
    Literal(Literal<'a>),
}

#[derive(Debug, Clone)]
pub struct FunctionCall<'a> {
    pub ident: Span<'a>,
    pub arguments: Vec<(Span<'a>, Expression<'a>)>,
}

#[derive(Debug, Clone)]
pub enum Variable<'a> {
    RelativePath(Span<'a>),
    QuotedString(Span<'a>), // todo: make this Value(Value)?
    Reference(Span<'a>),
    // TemplateFile(TemplateFile),
    // FunctionCall()
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    String(Span<'a>),
    Float(Span<'a>, f64),
}

impl Literal<'_> {
    pub fn to_string(&self) -> String {
        match self {
            Literal::String(s) => s.fragment().to_string(),
            Literal::Float(_s, f) => f.to_string(),
        }
    }
}

impl <'a>Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// impl <'a>Into<String> for Literal<'a> {
//     fn into(self) -> String {
//         match self {
//             Literal::String(s) => s.fragment().to_string(),
//             Literal::Float(_s, f) => f.to_string(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Element<'a> {
    pub ident: Span<'a>,
    pub attributes: Vec<(Span<'a>, Expression<'a>)>,
}

#[derive(Debug, Clone)]
pub enum StringToken<'a> {
    Text(Span<'a>),
    Variable(Variable<'a>),
}

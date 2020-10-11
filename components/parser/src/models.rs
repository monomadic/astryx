use crate::Span;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token {
//     Blank,
//     Letter,
// }

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    FunctionCall(FunctionCall<'a>),
    Element(Element<'a>),
}

#[derive(Debug, Clone)]
pub struct FunctionCall<'a> {
    pub ident: Span<'a>,
    pub arguments: Vec<(Span<'a>, Variable<'a>)>,
}

#[derive(Debug, Clone)]
pub enum Variable<'a> {
    RelativePath(Span<'a>),
    QuotedString(Span<'a>),
    Reference(Span<'a>),
    // TemplateFile(TemplateFile),
    // FunctionCall()
}

#[derive(Debug, Clone)]
pub struct Element<'a> {
    pub ident: Span<'a>,
    pub attributes: Vec<(Span<'a>, Span<'a>)>,
}

use crate::Span;

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
pub struct Element<'a> {
    pub ident: Span<'a>,
    pub attributes: Vec<(Span<'a>, Variable<'a>)>,
}

#[derive(Debug, Clone)]
pub enum StringToken<'a> {
    Text(Span<'a>),
    Variable(Variable<'a>),
}

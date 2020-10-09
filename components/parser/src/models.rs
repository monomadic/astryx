use crate::Span;

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    FunctionCall(FunctionCall<'a>)
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

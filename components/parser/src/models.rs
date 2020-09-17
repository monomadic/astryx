
#[derive(Debug, Clone)]
pub enum Token {
    // CodeBlock(CodeBlock),
    Comment(String),
    // Element(Element),
    // ForLoop(ForLoop),
    // FunctionCall(FunctionCall),
    // Text(Vec<StringToken>),
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub ident: String,
    pub content: String,
}

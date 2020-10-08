// Lexer
// Tokenises an astryx syntax into an AST

enum Token {
    Symbol,
    QuotedString,
    OpenBracket,
    CloseBracket,
}

enum Statement {
    
}

pub fn function_call(tokens: Vec<Token>) -> GrammarError {
    tuple((
        
    ))
}

pub(crate) fn statement(s: Span) -> IResult<Span, Token> {
    alt((
        map(comment, |s| Token::Comment(String::from(*s.fragment()))),
        map(for_loop, |f| Token::ForLoop(f)),
        map(function_call, |f| Token::FunctionCall(f)),
        map(piped_string, |string_tokens| Token::Text(string_tokens)),
        // map(codeblock, |cb| Token::CodeBlock(cb)),
        map(element, |e| Token::Element(e)),
    ))(s)
}

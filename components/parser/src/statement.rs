use crate::{
    models::Statement,
    ParserError, element::element, function::function_call, Span, text::piped_string,
};
use nom::{
    branch::alt,
    combinator::{all_consuming, map},
    IResult,
};

// fn array<'a>(i: Span) -> IResult<Span, Span, ParserError<Span>> {
//     // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
//     tuple((char('['), alpha1, char(']')))(i)
//         .map(|(r, (_, ident, _))| (r, ident))
//         .map_err(|e| {
//             e.map(|(span, _kind)| ParserError {
//                 context: span,
//                 kind: ParserErrorKind::SyntaxError,
//                 pos: span.into(),
//             })
//         })
// }

// #[test]
// fn test_array() {
//     assert!(array(Span::new("[g]")).is_ok());
// }

pub(crate) fn statement<'a>(i: Span<'a>) -> IResult<Span, Statement<'a>, ParserError<Span<'a>>> {
    all_consuming(alt((
        map(function_call, |f| Statement::FunctionCall(f)),
        map(element, |e| Statement::Element(e)),
        map(piped_string, |e| Statement::Text(e)),
        // map(alpha1, |e| Statement::Element(e)),
    )))(i)
    .map_err(|e| {
        e.map(|s| ParserError {
            context: i, // we need to reset the context to the whole line
            kind: s.kind,
            pos: s.pos,
        })
    })
}

#[test]
fn test_statement() {
    assert!(statement(Span::new("")).is_err()); // do not allow blank lines to slip through
    assert!(statement(Span::new("g()")).is_ok());
}

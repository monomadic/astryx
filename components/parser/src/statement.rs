use crate::{
    element::element, function::function_call, models::Statement, text::piped_string,
    variable::literal, Expression, ParserError, Span,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, space1},
    combinator::{all_consuming, map},
    sequence::{terminated, tuple},
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
        // map(function_call, |f| Statement::FunctionCall(f)),
        map(binding, |(ident, expr)| Statement::Binding(ident, expr)),
        map(expression, |e| Statement::Expression(e)),
        map(element, |e| Statement::Element(e)),
        map(piped_string, |e| Statement::Text(e)),
        // map(alpha1, |e| Statement::Element(e)),
        // return_statement
    )))(i)
    .map_err(|e| {
        e.map(|s| ParserError {
            context: i, // we need to reset the context to the whole line
            kind: s.kind,
            pos: s.pos,
        })
    })
}

pub(crate) fn expression<'a>(i: Span<'a>) -> IResult<Span, Expression<'a>, ParserError<Span<'a>>> {
    alt((
        map(function_call, |f| Expression::FunctionCall(f)),
        map(literal, |v| Expression::Literal(v)),
        map(alphanumeric1, |s| Expression::Reference(s)),
    ))(i)
}

fn binding<'a>(i: Span<'a>) -> IResult<Span, (Span<'a>, Expression<'a>), ParserError<Span<'a>>> {
    tuple((
        tag("let"),
        space1,
        alphanumeric1,
        terminated(space0, tag("=")),
        space0,
        expression,
    ))(i)
    .map(|(r, (_, _, ident, _, _, expr))| (r, (ident, expr)))
}

#[test]
fn test_binding() {
    assert!(binding(Span::new("let a=5")).is_ok());
    // assert_eq!(binding(Span::new("let a=5")).unwrap().0.fragment().to_string(), "a");
    assert!(binding(Span::new("let a = 5")).is_ok());
    assert!(binding(Span::new("let print = print()")).is_ok());
    assert!(binding(Span::new("let print = fn print()")).is_ok());
    assert!(binding(Span::new("g()")).is_err());
}

#[test]
fn test_statement() {
    assert!(statement(Span::new("")).is_err()); // do not allow blank lines to slip through
    assert!(statement(Span::new("g()")).is_ok());
}

use crate::{
    error::{ParserErrorKind},
    models::{FunctionCall, Statement},
    ParserError,
};
use nom::{
    branch::alt,
    character::complete::{alpha0, alpha1, char},
    combinator::{all_consuming, cut, map},
    sequence::tuple,
    IResult,
};
use nom_locate::LocatedSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Blank,
    Letter,
}

type Span<'a> = LocatedSpan<&'a str>;

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

fn function_call<'a>(i: Span<'a>) -> IResult<Span, FunctionCall<'a>, ParserError<Span>> {
    // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
    tuple((alpha1, char('('), cut(alpha0), cut(char(')'))))(i)
        .map(|(r, (ident, _, _, _))| {
            (
                r,
                FunctionCall {
                    ident,
                    arguments: Vec::new(),
                },
            )
        })
        .map_err(|e| {
            e.map(|(span, _kind)| ParserError {
                context: span,
                kind: ParserErrorKind::SyntaxError,
                pos: span.into(),
            })
        })
}

#[test]
fn test_function_call() {
    assert!(function_call(Span::new("g()")).is_ok());

    // check ident Span
    let f: FunctionCall = function_call(Span::new("function()")).unwrap().1;
    assert_eq!(f.ident.to_string(), "function");
    assert_eq!(f.ident.location_line(), 1);
    assert_eq!(f.ident.location_offset(), 0);
    assert_eq!(f.ident.get_column(), 1);

    // check no-match with error
    let e = function_call(Span::new("g"));
    match e {
        Err(nom::Err::Error(_)) => (),
        _ => panic!("expected Error, got {:?}", e),
    };

    // check partial match with fail
    let e = function_call(Span::new("g(1)"));
    match e {
        Err(nom::Err::Failure(_)) => (),
        _ => panic!("expected Failure, got {:?}", e),
    };
}

pub(crate) fn statement<'a>(i: Span<'a>) -> IResult<Span, Statement<'a>, ParserError<Span>> {
    println!("statement {:?}", i);
    all_consuming(alt((
        map(function_call, |f| Statement::FunctionCall(f)),
        map(function_call, |f| Statement::FunctionCall(f)),
    )))(i)
}

#[test]
fn test_statement() {
    assert!(statement(Span::new("")).is_err()); // do not allow blank lines to slip through
    assert!(statement(Span::new("g()")).is_ok());
    // assert!(statement(Span::new("[g]")).is_ok());
}

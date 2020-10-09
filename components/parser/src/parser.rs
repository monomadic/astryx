use crate::{
    error::{ParserErrorKind, Position},
    ParserError, models::{Statement, FunctionCall},
};
use nom::{
    branch::alt,
    character::complete::{alpha0, alpha1, alphanumeric1, char, space1},
    combinator::{all_consuming, cut, map},
    error::ParseError,
    error::{convert_error, VerboseError},
    multi::{many0, many1},
    sequence::tuple,
    Err, IResult,
};
use nom_locate::LocatedSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Blank,
    Letter,
}

type Span<'a> = LocatedSpan<&'a str>;

// fn letter<'a>(i: &'a str) -> IResult<&'a str, Token, ParserError<&'a str>> {
//     // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
//     map(alpha1, |_s| Token::Letter)(i)
//     .map_err(|e| {
//         e.map(|(span, _kind)| ParserError {
//             context: span,
//             kind: ParserErrorKind::SyntaxError,
//             pos: Position {
//                 line: 0,
//                 offset: 0,
//                 column: 0,
//             },
//         })
//     })
// }

// fn token<'a>(i: &'a str) -> IResult<&str, Token, ParserError<&'a str>> {
//     alt((letter, map(space1, |s| Token::Blank)))(i)
// }

// pub(crate) fn statement<'a>(i: &'a str) -> IResult<&str, Vec<Token>, ParserError<&'a str>> {
//     all_consuming(many0(letter))(i)
// }

fn array<'a>(i: Span) -> IResult<Span, Span, ParserError<Span>> {
    // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
    tuple((char('['), alpha1, char(']')))(i)
        .map(|(r, (_, ident, _))| (r, ident))
        .map_err(|e| {
            e.map(|(span, _kind)| ParserError {
                context: span,
                kind: ParserErrorKind::SyntaxError,
                pos: span.into(),
            })
        })
}
#[test]
fn test_array() {
    assert!(array(Span::new("[g]")).is_ok());
}

fn function_call<'a>(i: Span<'a>) -> IResult<Span, FunctionCall<'a>, ParserError<Span>> {
    // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
    tuple((alpha1, char('('), cut(alpha0), cut(char(')'))))(i)
        .map(|(r, (ident, _, _, _))| (r, FunctionCall {
            ident,
            arguments: Vec::new(),
        }))
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

    let e = function_call(Span::new("g"));
    match e {
        Err(Err::Error(_)) => (),
        _ => panic!("expected Error, got {:?}", e),
    };

    let e = function_call(Span::new("g(1)"));
    match e {
        Err(Err::Failure(_)) => (),
        _ => panic!("expected Failure, got {:?}", e),
    };
}

pub(crate) fn statement<'a>(i: Span<'a>) -> IResult<Span, Statement<'a>, ParserError<Span>> {
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

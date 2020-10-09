use crate::{
    error::{ParserErrorKind, Position},
    ParserError,
};
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, space1, char, alpha0},
    combinator::{all_consuming, map, cut},
    error::{convert_error, VerboseError},
    error::{ ParseError},
    multi::{many1, many0},
    Err, IResult, sequence::tuple,
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


fn function_call<'a>(i: Span) -> IResult<Span, Span, ParserError<Span>> {
    // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
    tuple((alpha1, char('('), cut(alpha0), cut(char(')'))))(i)
        .map(|(r, (ident, _, _, _))| (r, ident))
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


pub(crate) fn statement<'a>(i: Span) -> IResult<Span, Span, ParserError<Span>> {
    all_consuming(alt((
        function_call,
        array,
    )))(i)
}

#[test]
fn test_statement() {
    let input = "g()";

    match statement(Span::new(input)) {
        Err(Err::Error(e)) => {
            println!("[ERROR] {:?}", e)
            // println!("[ERROR] {:?}", nom::error::context("expected statement, found ", function_call)(Span::new(input)))
        },
        Err(Err::Failure(e)) => println!("[FAILURE] {:?}", e),
        Ok(s) => println!("statement ok - {:?}", s),
        _ => (),
    };
}

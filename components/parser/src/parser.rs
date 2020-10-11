use crate::{
    error::{ParserErrorKind},
    models::{FunctionCall, Statement},
    ParserError, Variable,
};
use nom::{
    branch::alt,
    character::complete::{alpha0, alpha1, char},
    combinator::{all_consuming, cut, map},
    sequence::tuple,
    IResult, multi::many0, bytes::complete::tag,
};
use nom_locate::{position, LocatedSpan};

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

fn function_call_argument<'a>(i: Span<'a>) -> IResult<Span<'a>, (Span<'a>, Variable<'a>), ParserError<Span<'a>>> {
    tuple((
        alpha0,
        tag(":"),
        alpha0,
    ))
    (i)
    .map(|(r, (ident, _, value))|
        (r, (ident, Variable::QuotedString(value)))
    )
}

fn function_call_arguments<'a>(i: Span<'a>) -> IResult<Span<'a>, Vec<(Span<'a>, Variable<'a>)>, ParserError<Span<'a>>> {
    many0(function_call_argument)(i)
}

fn function_call<'a>(i: Span<'a>) -> IResult<Span<'a>, FunctionCall<'a>, ParserError<Span<'a>>> {
    tuple((alpha1, char('('), cut(function_call_arguments), cut(char(')'))))(i)
        .map(|(r, (ident, _, arguments, _))| {
            (
                r,
                FunctionCall {
                    ident,
                    arguments,
                },
            )
        })
        .map_err(|e| e.map(|s| {
            ParserError {
                context: i,
                kind: ParserErrorKind::SyntaxError,
                pos: s.context.into(),
            }
        }))
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

use crate::{
    error::{ParserErrorKind, Position},
    ParserError,
};
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, space1},
    combinator::{all_consuming, map, cut},
    error::{convert_error, VerboseError},
    error::{ContextError, ParseError},
    multi::many0,
    Err, IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Blank,
    Letter,
}

fn letter<'a>(i: &'a str) -> IResult<&'a str, Token, ParserError<&'a str>> {
    // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
    map(alpha1, |_s| Token::Letter)(i)
    .map_err(|e| {
        e.map(|(span, _kind)| ParserError {
            context: span,
            kind: ParserErrorKind::SyntaxError,
            pos: Position {
                line: 0,
                offset: 0,
                column: 0,
            },
        })
    })
}

fn token<'a>(i: &'a str) -> IResult<&str, Token, ParserError<&'a str>> {
    alt((letter, map(space1, |s| Token::Blank)))(i)
}

pub(crate) fn statement<'a>(i: &'a str) -> IResult<&str, Vec<Token>, ParserError<&'a str>> {
    all_consuming(many0(letter))(i)
}

#[test]
fn test_statement() {
    let input = "za z z%";

    // let e = statement(input).unwrap_err();
    // println!(
    //     "verbose errors - `root::<VerboseError>(data)`:\n{}",
    //     convert_error(input, e.into())
    // );

    match statement(input) {
        Err(Err::Error(e)) => println!("Error {:?}", nom::error::context("mycon", statement)(input)),
        Err(Err::Failure(e)) => println!("Failure {:?}", e),
        Ok(s) => println!("statement ok - {:?}", s),
        _ => (),
    };
}

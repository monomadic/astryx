//! This crate parses astryx source and emits an AST (abstract syntax tree).
//!
//! There are a few stages to this:
//! 1. Lexical analysis: breaks up the raw text into tokens
//! 2. Parsing: transforms tokens into the AST
//!
//! If you wanted to add or change the syntax (language) of astryx,
//! everything you need is in this crate.
//!
//! ## Usage
//! ```
//! use parser;
//!
//! let source = "page\n";
//! let ast = parser::parse(source).unwrap();
//!
//! ```

use error::{ParserErrorKind, Position};
use nom::{
    branch::alt, character::complete::multispace1, error::VerboseError, sequence::tuple, IResult,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

pub mod error;
pub mod models;
pub mod parser;
pub mod variable;
pub use crate::error::{ParserError, ParserResult};
pub use crate::parser::Token;
mod eof;
mod linesplit;

/// returns a nom combinator version of the parser
pub fn _run(i: &str) -> IResult<Span, Vec<Token>> {
    // pub fn run<'a, S: Into<&'a str>>(i: S) -> IResult<Span<'a>, Vec<Token>> {
    tuple((
        nom::multi::many0(parser::node),
        alt((eof::eof, multispace1)),
    ))(Span::new(i))
    .map(|(r, (a, _))| (r, a))
}

pub fn __run(i: &str) -> ParserResult<Vec<Token>> {
    tuple((
        nom::multi::many0(parser::node),
        alt((eof::eof, multispace1)),
    ))(Span::new(i))
    .map(|(_, (a, _))| a)
    .map_err(|e| e.into())
}

pub fn run(i: &str) -> ParserResult<Vec<Token>> {
    let (_, lines) = linesplit::take_lines(i)?;

    let tokens: Vec<Token> = lines
        .iter()
        .flat_map(|line| {
            println!("sending {:?}", line);
            parser::node(line.content)
        })
        .map(|(_, token)| token)
        .collect();

    println!("{:?}", tokens);
    Ok(tokens)
}

#[test]
fn test_run() {
    assert!(run("").is_ok());
    // assert!(run("page").is_ok());
    assert!(run("page\n").is_ok());
    assert!(run("page\n\tdiv\n").is_ok());
    // assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 1);

    let result = run("hello\n@@@\n");
    println!("{:?}", result);

    // assert!(run("44").is_err());
}

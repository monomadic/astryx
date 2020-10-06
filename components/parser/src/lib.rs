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

use nom::{sequence::tuple, IResult, branch::alt, character::complete::multispace1};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

pub mod error;
pub mod parser;
pub mod variable;
pub mod models;
pub use crate::parser::Token;
pub use crate::error::{ParserError, ParserResult};
mod eof;

/// returns a nom combinator version of the parser
pub fn run(i: &str) -> IResult<Span, Vec<Token>> {
// pub fn run<'a, S: Into<&'a str>>(i: S) -> IResult<Span<'a>, Vec<Token>> {
    tuple((
        nom::multi::many0(parser::node),
        alt((eof::eof, multispace1))
    ))(Span::new(i))
    .map(|(r, (a, _))| (r, a))
}

#[test]
fn test_run() {
    assert!(run("").is_ok());
    // assert!(run("page").is_ok());
    assert!(run("page\n").is_ok());
    assert!(run("page\n\tdiv\n").is_ok());
    assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 1);

    let result = run("@@@\n");
    println!("{:?}", result);

    // assert!(run("44").is_err());
}

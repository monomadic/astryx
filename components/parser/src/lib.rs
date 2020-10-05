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

use nom::IResult;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

pub mod error;
pub mod parser;
pub mod variable;
pub mod models;
pub use crate::parser::Token;
pub use crate::error::{ParserError, ParserResult};

/// returns a nom combinator version of the parser
pub fn run<'a, S: Into<&'a str>>(i: S) -> IResult<Span<'a>, Vec<Token>> {
    nom::multi::many0(parser::node)(Span::new(&i.into()))
}

#[test]
fn test_run() {
    assert!(run("").is_ok());
    assert!(run("page").is_ok());
    assert!(run("page\n").is_ok());
    assert!(run("page\n\tdiv\n").is_ok());
    assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 1);
}

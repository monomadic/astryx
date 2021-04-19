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
//! let ast = parser::run(source).unwrap();
//!
//! ```

use linesplit::Line;
use nom::Err;
use nom_locate::LocatedSpan;
use rctree::Node;

mod element;
pub mod errorold;
mod function;
mod linesplit;
pub mod models;
pub mod statement;
mod text;
mod variable;
pub use crate::errorold::ParserError;
mod whitespace;
pub use crate::models::*;
use error::AstryxError;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub type ParserResult<T, I> = Result<T, ParserError<I>>;

/// Parses a tree of Spans into a tree of Statements.
pub fn parse<'a>(lines: Vec<Node<Span<'a>>>) -> Result<Vec<Node<Statement<'a>>>, AstryxError> {
    lines
        .into_iter()
        .map(statement::statement_node)
        .collect::<Result<Vec<(Span, Node<Statement<'_>>)>, nom::Err<ParserError<Span<'a>>>>>()
        .map(|result| {
            result
                .into_iter()
                .map(|(_, statements)| statements)
                .collect() // todo: if there are remainders, throw an error so this map is not required
        })
        .map_err(|e| match e {
            // convert to a regular result, nom is awful in this situation.
            Err::Error(e) | Err::Failure(e) => AstryxError::Generic("parser".into()),
            _ => unreachable!(),
        })
}

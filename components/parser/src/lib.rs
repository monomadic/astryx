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
use nom_locate::LocatedSpan;

use nom::{Err, IResult};
use rctree::Node;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub type ParserResult<T, I> = Result<T, ParserError<I>>;

mod element;
pub mod error;
mod function;
mod linesplit;
pub mod models;
pub mod statement;
mod text;
mod variable;
pub use crate::error::ParserError;
mod whitespace;
pub use crate::models::*;
use std::panic::Location;

pub fn parse<'a>(
    lines: Vec<Node<Span<'a>>>,
) -> Result<Vec<Node<Statement<'a>>>, ParserError<Span<'a>>> {
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
            Err::Error(e) | Err::Failure(e) => e,
            _ => unreachable!(),
        })
}

// this api absolutely needs a cleanup
// todo: take a PathBuf for filename, if it is actually needed.
pub fn run<'a>(
    i: &'a str,
    filename: &'a str,
) -> Result<Vec<Node<Statement<'a>>>, ParserError<Span<'a>>> {
    // let span = Span::new(i);
    let (_, lines): (_, Vec<Line>) =
        linesplit::take_lines(Span::new_extra(i, filename)).expect("linesplit fail (fix)"); // break document up by whitespace indentation

    lines
        .into_iter()
        .map(parse_line)
        .collect::<Result<Vec<(Span, Node<Statement<'_>>)>, nom::Err<ParserError<Span<'a>>>>>()
        .map_err(|e| match e {
            // convert to a regular result, nom is awful in this situation.
            Err::Error(e) | Err::Failure(e) => e,
            _ => unreachable!(),
        })
        // now we need to get rid of the remainder inside the result (I know, all of this is messy,
        // but it's isolated to this one function jumping from nom-style to rust-style.
        .map(|result| result.into_iter().map(|(_, nodes)| nodes).collect())
}

fn parse_line<'a>(line: Line<'a>) -> IResult<Span<'a>, Node<Statement<'a>>, ParserError<Span<'a>>> {
    let (r, statement) = statement::statement(line.content)?;
    let mut node: Node<Statement> = Node::new(statement);

    for child in line.children {
        let (_, child_node) = parse_line(child)?;
        node.append(child_node);
    }

    Ok((r, node))
}

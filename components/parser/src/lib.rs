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

use linesplit::Line;
use nom_locate::LocatedSpan;

use nom::{Err, IResult};
use rctree::Node;

pub type Span<'a> = LocatedSpan<&'a str>;
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
pub use crate::models::*;

// pub fn parse_line<'a>(i: Span<'a>) -> Result<Statement<'a>, ParserError<Span>> {
//     statement::statement(i)
//         .map(|(_r, result)| result)
//         .map_err(|e| match e {
//             Err::Error(e) | Err::Failure(e) => e,
//             Err::Incomplete(_) => unreachable!(),
//         })
// }

// #[test]
// fn test_parse_line() {
//     assert!(parse_line(Span::new("func()")).is_ok());
//     // assert!(parse_line(Span::new("func(a:1)")).is_ok());
//     assert!(parse_line(Span::new("func()\nfunc()")).is_err());
// }

pub fn run<'a>(i: &'a str) -> Result<Vec<Node<Statement<'a>>>, ParserError<Span<'a>>> {
    let (_, lines): (_, Vec<Line>) = linesplit::take_lines(&i).expect("linesplit fail (fix)"); // break document up by whitespace indentation

    // let l: Result<Vec<(Span, Node<Statement<'a>>)>, nom::Err<ParserError<Span<'a>>>> = lines
    //     .into_iter()
    //     .map(parse_line)
    //     .collect();

    // println!("lines: {:?}", i);

    lines
        .into_iter()
        .map(parse_line)
        .collect::<Result<Vec<(Span, Node<Statement<'a>>)>, nom::Err<ParserError<Span<'a>>>>>()
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
    // .map_err(|e| {
    //     e.map(|s| ParserError {
    //         context: line.content, // we need to reset the context to the whole line
    //         kind: ParserErrorKind::UnexpectedToken("6".into()),
    //         pos: s.pos,
    //     })
    // })?;
    // .map_err(|e| e.map(ParserError::from))?; // fix this

    let mut node: Node<Statement> = Node::new(statement);

    for child in line.children {
        // println!("line {:?}", &child.content);
        // let (_, statement) = statement::statement(child.content).unwrap();
        let (_, child_node) = parse_line(child)?;
        // println!("statement {:?}", &statement);
        node.append(child_node);
    }

    Ok((r, node))
}

#[test]
fn test_run() {
    assert!(run("").is_ok());
    // assert!(run("page").is_ok());
    assert!(run("page()\n").is_ok());
    assert!(run("page()\ndiv()\n").is_ok());
    // assert!(run("page()\n\tdiv\n").is_err()); // children
    // assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 1);

    // let result = run("hello\n@@@\n");
    // println!("{:?}", result);

    // println!("--{:?}", run("func()\n"));

    // assert!(run("44").is_err());
}

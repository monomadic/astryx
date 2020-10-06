// use std::error::Error;
// use std::fmt;

use nom::{Err, IResult};
use crate::Span;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub pos: Position,
    pub context: String,
}

#[derive(Debug)]
pub struct Position {
    pub line: u32,
    pub column: usize,
    pub offset: usize,
}

impl From<Err<(nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)>> for ParserError {
    fn from(err: Err<(nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)>) -> ParserError {
        // ParserError {
        //     kind: ParserErrorKind::SyntaxError,
        //     pos: Position {
        //         line: err.0.span.location_line(),
        //         offset: err.0.span.location_offset(),
        //         column:err.0.span.get_column(),
        //     }
        // }

        match err {
            Err::Error((span, _kind)) |
            Err::Failure((span, _kind)) => ParserError {
                kind: ParserErrorKind::SyntaxError,
                pos: Position {
                    line: span.location_line(),
                    offset: span.location_offset(),
                    column: span.get_column(),
                },
                context: String::from(*span.fragment())
            },
            _ => unimplemented!(),
        }
    }
}

// impl ParserError {
//     pub fn from_nom_error(err: nom::Err))
//     pub fn new(_msg: &str) -> Self {
//         unimplemented!()
//     }
// }

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
    SyntaxError
}

// impl Error for ParseError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }

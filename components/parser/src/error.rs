// use std::error::Error;
// use std::fmt;

use crate::Span;
use nom::{
    error::{ErrorKind, ParseError},
    Err,
};

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub pos: Position,
    pub context: String, // this is probably not necessary
}

#[derive(Debug)]
pub struct Position {
    pub line: u32,
    pub column: usize,
    pub offset: usize,
}

impl<I> ParseError<I> for ParserError {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        // ParserError::Unhandled
        panic!("unhandled error");
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

// this is selfish and a perf hit, but I don't want to expose Span
// it's not that bad as these aren't heap allocated
impl<'a> From<Span<'a>> for Position {
    fn from(span: Span) -> Position {
        Position {
            line: span.location_line(),
            offset: span.location_offset(),
            column: span.get_column(),
        }
    }
}

// used by the main api interface, run()
impl From<Err<(nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)>> for ParserError {
    fn from(err: Err<(nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)>) -> ParserError {
        println!("err: {:?}", err);
        match err {
            Err::Error((span, _kind)) | Err::Failure((span, _kind)) => ParserError {
                context: span.to_string(),
                kind: ParserErrorKind::SyntaxError,
                pos: span.into(),
            },
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
}

impl From<(nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)> for ParserError {
    fn from(err: (nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)) -> ParserError {
        let (span, kind) = err;
        ParserError {
            context: span.to_string(),
            kind: ParserErrorKind::SyntaxError,
            pos: span.into(),
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
    SyntaxError,
    FunctionArgumentError,
    Unhandled,
}

// impl Error for ParseError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }

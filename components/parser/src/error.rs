use crate::Span;
use nom::error::{ErrorKind, ParseError};

#[derive(Debug)]
pub struct ParserError<I> {
    pub kind: ParserErrorKind<I>,
    pub pos: I,
    pub context: I,
}

impl<'a> ParseError<Span<'a>> for ParserError<Span<'a>> {
    fn from_error_kind(input: Span<'a>, _kind: ErrorKind) -> Self {
        // panic!("incoming: {:?}", input);
        ParserError {
            kind: ParserErrorKind::Unhandled,
            pos: input,
            context: input,
        }
    }

    fn append(_i: Span<'a>, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind<I> {
    SyntaxError,
    FunctionArgumentError,
    Unhandled,
    UnexpectedToken(String),
    ExpectedValue,
    Nom(I),
}

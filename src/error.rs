use parser::{ParserError, Span};
use interpreter::InterpreterError;
use html::HTMLError;

pub type AstryxResult<'a, T> = Result<T, AstryxError<'a>>;

#[derive(Debug)]
pub enum AstryxError<'a> {
    ParserError(ParserError<Span<'a>>),
    InterpreterError,
    HTMLError,
}

impl <'a>From<ParserError<Span<'a>>> for AstryxError<'a> {
    fn from(e: ParserError<Span<'a>>) -> AstryxError<'a> {
        AstryxError::ParserError(e)
    }
}

impl <'a>From<InterpreterError> for AstryxError<'a> {
    fn from(_e: InterpreterError) -> AstryxError<'a> {
        AstryxError::InterpreterError
    }
}

impl <'a>From<HTMLError> for AstryxError<'a> {
    fn from(_e: HTMLError) -> AstryxError<'a> {
        AstryxError::HTMLError
    }
}

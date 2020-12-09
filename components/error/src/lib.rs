use parser::{ParserError, Span};

pub type AstryxResult<T> = Result<T, AstryxError>;

#[derive(Debug)]
pub enum AstryxError {
    LocatedError(Location, AstryxErrorKind),
    // HTMLError,
    IO(std::io::Error),
}

#[derive(Debug, PartialEq)]
pub enum AstryxErrorKind {
    SyntaxError,
    FunctionArgumentError,
    Unexpected,
    UnexpectedToken(String),
    ExpectedValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub line: u32,
    pub column: usize,
    pub length: usize,
}

impl<'a> From<Span<'a>> for Location {
    fn from(span: Span) -> Self {
        Self {
            line: span.location_line(),
            column: span.get_column(),
            length: span.location_offset(),
        }
    }
}

impl<'a> From<ParserError<Span<'a>>> for AstryxError {
    fn from(e: ParserError<Span<'a>>) -> AstryxError {
        AstryxError::LocatedError(e.pos.into(), AstryxErrorKind::Unexpected) // FIXME
    }
}

impl<'a> From<std::io::Error> for AstryxError {
    fn from(e: std::io::Error) -> AstryxError {
        AstryxError::IO(e)
    }
}

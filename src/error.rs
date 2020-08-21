use crate::interpreter::State;
use parser::error::ParserError;

pub type AstryxResult<T> = Result<T, AstryxError>;

#[derive(Debug, Clone)]
pub struct AstryxError {
    pub kind: AstryxErrorKind,
    pub state: Option<State>,
    pub msg: String,
}

#[derive(Debug, Clone)]
pub enum AstryxErrorKind {
    Unknown,
    ParseError,
    InterpreterError,
    IOError,
    ServerError,
    UnrecognisedElement(String)
}

impl AstryxError {
    pub fn new(msg: &str) -> AstryxError {
        Self {
            kind: AstryxErrorKind::Unknown,
            state: None,
            msg: msg.into(),
        }
    }
}

impl From<ParserError> for AstryxError {
    fn from(err: ParserError) -> Self {
        AstryxError {
            kind: AstryxErrorKind::ParseError,
            state: None,
            msg: err.msg,
        }
    }
}

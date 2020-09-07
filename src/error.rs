// use crate::interpreter::State;
use parser::error::ParserError;

pub type AstryxResult<T> = Result<T, AstryxError>;

#[derive(Debug, Clone)]
pub struct AstryxError {
    pub kind: AstryxErrorKind,
    // pub state: Option<State>,
    pub msg: String,
}

#[derive(Debug, Clone)]
pub enum AstryxErrorKind {
    Unknown,
    ParseError,
    InterpreterError,
    IOError,
    ServerError,
    UndefinedVariable(String),
    MissingRequiredArgument(String),
    FilesNotFound(String),
    UnrecognisedElement(String)
}

impl AstryxError {
    pub fn new<S:ToString>(msg: S) -> AstryxError {
        Self {
            kind: AstryxErrorKind::Unknown,
            // state: None,
            msg: msg.to_string(),
        }
    }

    pub fn new_from(kind: AstryxErrorKind) -> AstryxError {
        Self {
            kind,
            msg: String::new(),
        }
    }
}

impl From<ParserError> for AstryxError {
    fn from(err: ParserError) -> Self {
        AstryxError {
            kind: AstryxErrorKind::ParseError,
            // state: None,
            msg: err.msg,
        }
    }
}

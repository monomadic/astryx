// use crate::interpreter::State;
use parser::error::ParserError;
use std::path::PathBuf;

pub type AstryxResult<T> = Result<T, AstryxError>;

// #[derive(Debug, Clone)]
pub struct AstryxError {
    pub kind: AstryxErrorKind,
    // pub state: Option<State>,
    pub msg: String,
}

// #[derive(Debug, Clone)]
pub enum AstryxErrorKind {
    FileNotFound(String),
    FilesNotFound(String),
    CannotReadFile(String),
    InterpreterError,
    IOError(std::io::Error),
    MissingRequiredArgument(String),
    ParseError,
    ServerError,
    UndefinedVariable(String),
    UnexpectedFunction(String),
    Unknown,
    UnrecognisedElement(String),
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

impl std::fmt::Display for AstryxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AstryxErrorKind::FileNotFound(s) => write!(f, "File Not Found: {}", s),  
            AstryxErrorKind::FilesNotFound(s) => write!(f, "Files Not Found: {}", s),
            AstryxErrorKind::InterpreterError => unimplemented!(),
            AstryxErrorKind::IOError(e) => write!(f, "I/O Error: {}", e),
            AstryxErrorKind::MissingRequiredArgument(ident) => write!(f, "Missing Required Argument: {}", ident),
            AstryxErrorKind::ParseError => unimplemented!(),
            AstryxErrorKind::ServerError => unimplemented!(),
            AstryxErrorKind::UndefinedVariable(_) => unimplemented!(),
            AstryxErrorKind::UnexpectedFunction(_) => unimplemented!(),
            AstryxErrorKind::Unknown => write!(f, "Unknown Error: {}", self.msg),
            AstryxErrorKind::UnrecognisedElement(_) => unimplemented!(),
            _ => unimplemented!()
        }
    }
}

impl From<std::io::Error> for AstryxError {
    fn from(error: std::io::Error) -> Self {
        AstryxError::new_from(AstryxErrorKind::IOError(error))
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

// use parser::error::ParserError;

use parser::{ParserError};

pub type AstryxResult<T> = Result<T, AstryxError>;

#[derive(Debug)]
pub enum AstryxError {
    ParserError(ParserError)
}

impl From<ParserError> for AstryxError {
    fn from(e: ParserError) -> AstryxError {
        AstryxError::ParserError(e)
    }
}

#[derive(Debug)]
pub enum AstryxErrorKind {
    FileNotFound(String),
    FilesNotFound(String),
    CannotReadFile(String),
    InterpreterError,
    IOError(std::io::Error),
    MissingRequiredArgument(String),
    ParserError,
    ServerError,
    UndefinedVariable(String),
    UnexpectedFunction(String),
    Unknown,
    UnrecognisedElement(String),
}

impl AstryxError {
    pub fn new<S:ToString>(msg: S) -> AstryxError {
        unimplemented!()
    }

    pub fn new_from(kind: AstryxErrorKind) -> AstryxError {
        unimplemented!()
    }
}

impl std::fmt::Display for AstryxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
        // match &self.kind {
        //     AstryxErrorKind::FileNotFound(s) => write!(f, "File Not Found: {}", s),  
        //     AstryxErrorKind::FilesNotFound(s) => write!(f, "Files Not Found: {}", s),
        //     AstryxErrorKind::InterpreterError => unimplemented!(),
        //     AstryxErrorKind::IOError(e) => write!(f, "I/O Error: {}", e),
        //     AstryxErrorKind::MissingRequiredArgument(ident) => write!(f, "Missing Required Argument: {}", ident),
        //     AstryxErrorKind::ParserError => unimplemented!(),
        //     AstryxErrorKind::ServerError => unimplemented!(),
        //     AstryxErrorKind::UndefinedVariable(_) => unimplemented!(),
        //     AstryxErrorKind::UnexpectedFunction(_) => unimplemented!(),
        //     AstryxErrorKind::Unknown => write!(f, "Unknown Error"),
        //     AstryxErrorKind::UnrecognisedElement(_) => unimplemented!(),
        //     _ => unimplemented!()
        // }
    }
}

impl From<std::io::Error> for AstryxError {
    fn from(error: std::io::Error) -> Self {
        AstryxError::new_from(AstryxErrorKind::IOError(error))
    }
}

// impl From<ParserError> for AstryxError {
//     fn from(err: ParserError) -> Self {
//         AstryxError {
//             kind: AstryxErrorKind::ParserError,
//             // state: None,
//             msg: "".into(),
//         }
//     }
// }

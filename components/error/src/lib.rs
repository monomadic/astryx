use crate::display::display_error;
use quick_error::quick_error;

pub mod display;

pub type AstryxResult<T> = Result<T, AstryxError>;

quick_error! {
    #[derive(Debug)]
    pub enum AstryxError {
        /// An error with an associated file context
        LocatedError(location: Location, kind: AstryxErrorKind) {
        } // todo: should contain a pathbuf for source file

        /// A generic error without context
        Generic(err: String) {
            display("generic error: {}", err)
        }

        /// File or IO error
        IO(err: std::io::Error) {
            from()
        }
    }
}

impl AstryxError {
    pub fn output(self) -> String {
        display_error(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum AstryxErrorKind {
    SyntaxError,
    FunctionArgumentError,
    MissingRequiredArgument(String),
    Unexpected,
    UnexpectedToken(String),
    ExpectedValue,
    UnknownValue(String),
    Unimplemented(String),
    FilePatternError(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub line: u32,
    pub column: usize,
    pub length: usize,
    pub filename: String,
    pub context: String,
}

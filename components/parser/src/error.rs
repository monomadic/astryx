// use std::error::Error;
// use std::fmt;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    // pub location: Location,
    pub msg: String,
}

impl ParserError {
    pub fn new(msg: &str) -> Self {
        ParserError {
            kind: ParserErrorKind::Generic,
            msg: msg.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
    Generic
}

// impl Error for ParseError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }

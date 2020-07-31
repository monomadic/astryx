use crate::interpreter::State;

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

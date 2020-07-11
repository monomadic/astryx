use crate::interpreter::State;

// TODO this should be GizmoResult or something
pub type AstryxResult<T> = Result<T, AstryxError>;

#[derive(Debug, Clone)]
pub struct AstryxError {
    kind: AstryxErrorKind,
    state: Option<State>,
    msg: String,
}

#[derive(Debug, Clone)]
pub enum AstryxErrorKind {
    Unknown,
    ParseError,
}

impl AstryxError {
    pub fn new(msg: &str) -> AstryxError {
        Self {
            kind: AstryxErrorKind::Unknown,
            state: None,
            msg: msg.into()
        }
    }
}

// impl error::Error for CassetteError {
//     // fn description(&self) -> &str {
//     //     &format!("{}", self)
//     // }
// }

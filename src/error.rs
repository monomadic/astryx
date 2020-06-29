use std::error::Error;

// TODO this should be GizmoResult or something
pub type ParseResult<T> = Result<T, CassetteError>;

#[derive(Debug, Clone)]
pub enum CassetteError {
    ParseError(String),
}

// impl error::Error for CassetteError {
//     // fn description(&self) -> &str {
//     //     &format!("{}", self)
//     // }
// }

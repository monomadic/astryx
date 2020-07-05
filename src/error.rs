// TODO this should be GizmoResult or something
pub type ParseResult<T> = Result<T, AstryxError>;

#[derive(Debug, Clone)]
pub enum AstryxError {
    ParseError(String),
}

// impl error::Error for CassetteError {
//     // fn description(&self) -> &str {
//     //     &format!("{}", self)
//     // }
// }

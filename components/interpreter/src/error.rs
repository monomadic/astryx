#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unhandled,
    Generic(String),
    NoWriter,
    FunctionNotFound(String),
    ReferenceIsNotAFunction,
    InvalidReference(String),
}

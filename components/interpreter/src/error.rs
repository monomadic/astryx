#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unhandled,
    Generic(String),
    NoWriter,
}

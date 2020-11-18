#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unhandled,
    Generic(String),
    NoWriter,
    FunctionNotFound(String),
    UnknownMemberFunction(String),
    ReferenceIsNotAFunction,
    InvalidReference(String),
    UnexpectedToken { expected: String, got: String },
    IOError,
}

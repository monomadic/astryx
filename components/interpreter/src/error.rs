use parser::Span;

#[derive(Debug, PartialEq)]
pub struct InterpreterError {
    pub kind: InterpreterErrorKind,
    pub location: Option<Location>,
}

#[derive(Debug, PartialEq)]
pub struct Location {
    line: u32,
    column: usize,
    length: usize,
}

impl<'a> From<Span<'a>> for Location {
    fn from(span: Span) -> Self {
        Self {
            line: span.location_line(),
            column: span.get_column(),
            length: span.location_offset(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InterpreterErrorKind {
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

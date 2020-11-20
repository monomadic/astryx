use parser::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct InterpreterError {
    pub kind: InterpreterErrorKind,
    pub location: Option<Location>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub line: u32,
    pub column: usize,
    pub length: usize,
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

#[derive(Debug, PartialEq, Clone)]
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

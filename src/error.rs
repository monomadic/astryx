use parser::{ParserError, Span, error::Position};
use interpreter::InterpreterError;
use html::HTMLError;

pub type AstryxResult<'a, T> = Result<T, AstryxError<'a>>;

#[derive(Debug)]
pub enum AstryxError<'a> {
    ParserError(ParserError<Span<'a>>),
    InterpreterError,
    HTMLError,
    IO(std::io::Error)
}

impl <'a>From<ParserError<Span<'a>>> for AstryxError<'a> {
    fn from(e: ParserError<Span<'a>>) -> AstryxError<'a> {
        AstryxError::ParserError(e)
    }
}

impl <'a>From<InterpreterError> for AstryxError<'a> {
    fn from(_e: InterpreterError) -> AstryxError<'a> {
        AstryxError::InterpreterError
    }
}

impl <'a>From<HTMLError> for AstryxError<'a> {
    fn from(_e: HTMLError) -> AstryxError<'a> {
        AstryxError::HTMLError
    }
}

impl <'a>From<std::io::Error> for AstryxError<'a> {
    fn from(e: std::io::Error) -> AstryxError<'a> {
        AstryxError::IO(e)
    }
}

/// convert an error to a display-friendly string
pub(crate) fn display_error(err: AstryxError, path: &str) -> String {
    // println!("error: {:?}", err);
    match &err {
        AstryxError::ParserError(e) =>
            error_with_line(&e.pos, &e.context, "reason", path),
        AstryxError::InterpreterError => format!("InterpreterError"),
        AstryxError::HTMLError => format!("HTMLError"),
        AstryxError::IO(_) => format!("IO"),
    }
}

// terminal view for errors
fn error_with_line(pos: &Position, context: &Span, reason: &str, path: &str) -> String {
    [
        format!("error: {}", reason),
        format!("--> {}:{}:{}", path, pos.line, pos.column),
        String::from("  |"),
        format!("{} | {}", pos.line, context), //file.lines().into_iter().enumerate().collect::<Vec<String>>()[context.location_line() as usize]),
        format!(
            "  |{space:column$}{caret:offset$}",
            space = " ",
            caret = "^",
            column = pos.column,
            offset = pos.offset
        ),
    ]
    .join("\n")
}

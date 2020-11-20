// use html::HTMLError;
use interpreter::{InterpreterError, InterpreterErrorKind};
use parser::{error::ParserErrorKind, ParserError, Span};

pub type AstryxResult<'a, T> = Result<T, AstryxError<'a>>;

#[derive(Debug)]
pub enum AstryxError<'a> {
    ParserError(ParserError<Span<'a>>),
    InterpreterError(InterpreterError),
    // HTMLError,
    IO(std::io::Error),
}

impl<'a> From<ParserError<Span<'a>>> for AstryxError<'a> {
    fn from(e: ParserError<Span<'a>>) -> AstryxError<'a> {
        AstryxError::ParserError(e)
    }
}

impl<'a> From<InterpreterError> for AstryxError<'a> {
    fn from(e: InterpreterError) -> AstryxError<'a> {
        AstryxError::InterpreterError(e)
    }
}

// impl<'a> From<HTMLError> for AstryxError<'a> {
//     fn from(_e: HTMLError) -> AstryxError<'a> {
//         AstryxError::HTMLError
//     }
// }

impl<'a> From<std::io::Error> for AstryxError<'a> {
    fn from(e: std::io::Error) -> AstryxError<'a> {
        AstryxError::IO(e)
    }
}

pub(crate) fn html_error_page(content: &str) -> String {
    format!("<html style='background-color: black;color: white;'><body><h1>Error :(</h1><pre>{}</pre></body></html>", content)
}

/// convert an error to a display-friendly string
pub(crate) fn display_error(err: &AstryxError, path: &str) -> String {
    match &err {
        AstryxError::ParserError(e) => format!(
            "syntax error: {}",
            error_with_line(
                &e.context.to_string(),
                e.pos.location_line(),
                e.pos.get_column() as usize,
                e.pos.location_offset() as usize,
                &parser_reason(&e.kind),
                path
            )
        ),
        AstryxError::InterpreterError(e) => {
            let location = e.clone().location.unwrap();
            format!(
                "logical error: {}",
                error_with_line(
                    "codeee",
                    location.line,
                    location.column,
                    location.length,
                    &interpreter_reason(&e.kind),
                    path
                )
            )
        }
        // AstryxError::HTMLError => format!("HTMLError"),
        AstryxError::IO(_) => format!("IO"),
    }
}

fn parser_reason(kind: &ParserErrorKind<Span>) -> String {
    format!("{:?}", kind)
}

fn interpreter_reason(kind: &InterpreterErrorKind) -> String {
    match &kind {
        InterpreterErrorKind::NoWriter => {
            format!("cannot write to output without a specified file or stdout target.")
        }
        InterpreterErrorKind::Unhandled => format!("unhandler interpreter error."),
        InterpreterErrorKind::Generic(e) => format!("{:?}", e),
        InterpreterErrorKind::FunctionNotFound(f) => format!("function not found {:?}", f),
        InterpreterErrorKind::ReferenceIsNotAFunction => format!("ReferenceIsNotAFunction"),
        InterpreterErrorKind::InvalidReference(r) => format!("invalid reference: {}", r),
        InterpreterErrorKind::UnexpectedToken { expected, got } => {
            format!("expected {}, got {}", expected, got)
        }
        InterpreterErrorKind::IOError => format!("error reading file"),
        InterpreterErrorKind::UnknownMemberFunction(mf) => {
            format!("missing member function: {}", mf)
        }
    }
}

// terminal view for errors
fn error_with_line(
    context: &str,
    line: u32,
    col: usize,
    length: usize,
    reason: &str,
    path: &str,
) -> String {
    // panic!("context: {:?}", context.fragment());
    [
        format!("{}", reason),
        format!("--> {}:{}:{}", path, line, col),
        String::from("  |"),
        format!("{} | {}", line, context), //file.lines().into_iter().enumerate().collect::<Vec<String>>()[context.location_line() as usize]),
        format!(
            "  |{space:column$}{caret:offset$}",
            space = " ",
            caret = "^",
            column = col,
            offset = length,
        ),
    ]
    .join("\n")
}

// // terminal view for errors
// fn error_with_line(context: &Span, pos: &Span, reason: &str, path: &str) -> String {
//     // panic!("context: {:?}", context.fragment());
//     [
//         format!("{}", reason),
//         format!("--> {}:{}:{}", path, pos.location_line(), pos.get_column()),
//         String::from("  |"),
//         format!("{} | {}", context.location_line(), context), //file.lines().into_iter().enumerate().collect::<Vec<String>>()[context.location_line() as usize]),
//         format!(
//             "  |{space:column$}{caret:offset$}",
//             space = " ",
//             caret = "^",
//             column = pos.get_column(),
//             offset = pos.location_offset(),
//         ),
//     ]
//     .join("\n")
// }

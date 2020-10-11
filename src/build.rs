use crate::error::*;
use parser::{error::Position, Span};

pub(crate) fn build<'a>(path: &str) -> AstryxResult<'a, ()> {
    let file = std::fs::read_to_string(path)?;

    match crate::render::render(&file) {
        Ok(buffers) => println!("ok. {:?}", buffers),
        Err(e) => display_error(e, &path, &file),
    }
    Ok(())
}

fn display_error(err: AstryxError, path: &str, file: &str) {
    // println!("error: {:?}", err);
    match &err {
        AstryxError::ParserError(e) => {
            println!("{}", error_with_line(&e.pos, &e.context, "reason", path, file))
        }
        AstryxError::InterpreterError => {}
        AstryxError::HTMLError => {}
        AstryxError::IO(_) => {}
    }
}

/// TODO: move this into another module...
fn error_with_line(pos: &Position, context: &Span, reason: &str, path: &str, file: &str) -> String {
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

// fn error_reason(e: &AstryxError) -> String {
//     match e {
//         AstryxError::ParserError(e) => {
//             match e.kind {
//                 ParserErrorKind::SyntaxError => "syntax error."
//             }
//         }
//     }.into()
// }

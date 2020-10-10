use crate::error::*;
use parser::{error::{Position}};

pub(crate) fn build<'a>(file: String) -> AstryxResult<'a, ()> {
    // match crate::render::render(&file) {
    //     Ok(buffers) => println!("ok. {:?}", buffers),
    //     Err(e) => display_error(e)
    // }
    Ok(())
}

fn display_error(err: AstryxError) {
    // println!("error: {:?}", err);
    match &err {
        AstryxError::ParserError(e) => println!("{}", error_with_line("test.src", &e.pos, &e.context.to_string(), "--")),
        AstryxError::InterpreterError => {}
        AstryxError::HTMLError => {}
    }
}

fn error_with_line(filename: &str, pos: &Position, context: &str, reason: &str) -> String {
    [
        format!("error: {}", reason),
        format!("--> {}:{}:{}", filename, pos.line, pos.column),
        String::from("  |"),
        format!("{} | {}  |", pos.line, context),
    ].join("\n")
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

use astryx::error::*;
use parser::{error::{ParserErrorKind, Position}};

pub(crate) fn build(file: String) -> AstryxResult<()> {
    match astryx::render(&file) {
        Ok(_) => println!("ok."),
        Err(e) => display_error(e)
    }
    Ok(())
}

fn display_error(err: AstryxError) {
    // println!("error: {:?}", err);
    match &err {
        AstryxError::ParserError(e) => println!("{}", error_with_line("test.src", &e.pos, &e.context, &error_reason(&err)))
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

fn error_reason(e: &AstryxError) -> String {
    match e {
        AstryxError::ParserError(e) => {
            match e.kind {
                ParserErrorKind::SyntaxError => "syntax error."
            }
        }
    }.into()
}

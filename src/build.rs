use crate::error::*;

pub(crate) fn build<'a>(path: &str) -> AstryxResult<'a, ()> {
    let file = std::fs::read_to_string(path)?;

    match crate::render::render(&file) {
        Ok(buffers) => println!("ok. {:#?}", buffers),
        Err(e) => println!("{}", display_error(e, &path)),
    }
    Ok(())
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

use crate::error::*;
use rctree::Node;

pub(crate) fn build<'a>(path: &str) -> AstryxResult<'a, ()> {
    let file = std::fs::read_to_string(path)?;

    match crate::render::render(&file) {
        Ok(buffers) => buffers.into_iter().for_each(|b| print_node(b)),
        Err(e) => println!("{}", display_error(&e, &path)),
    }
    Ok(())
}

fn print_node<T:std::fmt::Debug>(node: Node<T>) {
    println!("-- {:?}", node.borrow());
    for child in node.children() {
        print_node(child);
    }
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

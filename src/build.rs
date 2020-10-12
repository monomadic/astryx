use crate::error::*;
use rctree::Node;

pub(crate) fn build<'a>(path: &str) -> AstryxResult<'a, ()> {
    let file = std::fs::read_to_string(&path)?;

    let _ = crate::render::render(&file)
        .and_then(|nodes| interpreter::run(nodes).map_err(AstryxError::from))
        .map_err(AstryxError::from);

    // std::fs::read_to_string(&path)
    //     .map_err(AstryxError::from)
    //     .and_then(|file| crate::render::render(&file))
    //     .and_then(|nodes| interpreter::run(nodes).map_err(AstryxError::from))?;

    Ok(())
}

fn print_node<T: std::fmt::Debug>(node: Node<T>) {
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

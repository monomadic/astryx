use crate::error::*;
use rctree::Node;
use interpreter::State;

pub(crate) fn build<'a>(file: &'a str) -> AstryxResult<'a, ()> {
    let state = &mut State::new();
    crate::render::render(file)
        .and_then(|nodes| interpreter::run(nodes, state).map_err(AstryxError::from))
        .map_err(AstryxError::from)
        .map(|_| ()) // gotta terminate with an empty tuple so we still get the error.

    // std::fs::read_to_string(&path)
    //     .map_err(AstryxError::from)
    //     .and_then(|file| crate::render::render(&file))
    //     .and_then(|nodes| interpreter::run(nodes).map_err(AstryxError::from))?;

    // Ok(())
}

fn _print_node<T: std::fmt::Debug>(node: Node<T>) {
    println!("-- {:?}", node.borrow());
    for child in node.children() {
        _print_node(child);
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

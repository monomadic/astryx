use crate::error::*;
use interpreter::{State, Writer};
use rctree::Node;

pub(crate) fn build<'a>(file: &'a str) -> AstryxResult<'a, ()> {
    let state = &mut State::new();
    // state.writer = Writer::File("index.html".to_string());
    // state.writer = Writer::StdOut;

    parser::run(file)
        .map_err(AstryxError::from)
        .and_then(|nodes| interpreter::run(nodes, state).map_err(AstryxError::from))
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

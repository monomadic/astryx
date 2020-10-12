use crate::error::{AstryxError, AstryxResult};
use parser::Statement;
use rctree::Node;

pub trait RenderErrorAsHTML {
    fn to_html(&self) -> String;
}

impl RenderErrorAsHTML for AstryxError<'_> {
    fn to_html(&self) -> String {
        String::from("hi")
    }
}

// pub fn render<'a>(file: &'a str) -> AstryxResult<'a, std::collections::HashMap<String, String>> {
pub fn render<'a>(file: &'a String) -> AstryxResult<'a, Vec<Node<Statement<'a>>>> {
    parser::run(&file)
        .map_err(AstryxError::from)
        // .and_then(|ast: Vec<Statement>| interpreter::run(&ast).map_err(AstryxError::from))
        // .and_then(|html_nodes| {
        //     html::render_as_string(&html_nodes)
        //         .map_err(AstryxError::from)
        // })
        // .map_err(AstryxError::from)
}

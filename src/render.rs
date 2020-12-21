use error::{AstryxError, AstryxResult};
use models::State;
use std::cell::RefCell;
use std::rc::Rc;

pub trait RenderErrorAsHTML {
    fn to_html(&self) -> String;
}

impl RenderErrorAsHTML for AstryxError {
    fn to_html(&self) -> String {
        String::from("hi")
    }
}

// pub fn render<'a>(file: &'a str) -> AstryxResult<'a, std::collections::HashMap<String, String>> {
pub fn render<'a>(file: &'a str) -> AstryxResult<()> {
    parser::run(&file, "<error>")
        .map_err(|_| AstryxError::Generic("fixme".to_string()))
        .and_then(|nodes| interpreter::run(&nodes, Rc::new(RefCell::new(State::new()))))
        // .map(|p| program::render_project(p))
        .map_err(AstryxError::from)
        .map(|_| ())
    // .and_then(|html_nodes| {
    //     html::render_as_string(&html_nodes)
    //         .map_err(AstryxError::from)
    // })
    // .map_err(AstryxError::from)
}

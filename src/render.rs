use crate::error::{AstryxError, AstryxResult};
use interpreter::State;
use parser::Statement;
use program::Project;
use rctree::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub trait RenderErrorAsHTML {
    fn to_html(&self) -> String;
}

impl RenderErrorAsHTML for AstryxError<'_> {
    fn to_html(&self) -> String {
        String::from("hi")
    }
}

// pub fn render<'a>(file: &'a str) -> AstryxResult<'a, std::collections::HashMap<String, String>> {
pub fn render<'a>(file: &'a str) -> AstryxResult<'a, Project> {
    parser::run(&file)
        .map_err(AstryxError::from)
        .and_then(|nodes| {
            interpreter::run(&nodes, Rc::new(RefCell::new(State::new()))).map_err(AstryxError::from)
        })
        .map(|p| program::render_project(p))
        .map_err(AstryxError::from)
    // .and_then(|html_nodes| {
    //     html::render_as_string(&html_nodes)
    //         .map_err(AstryxError::from)
    // })
    // .map_err(AstryxError::from)
}

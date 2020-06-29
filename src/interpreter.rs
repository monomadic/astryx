use crate::error::*;
// use crate::functions::*;
use crate::models::*;

#[derive(Debug, Clone)]
pub struct Site {
    // pub styles: HashMap<String, Style>,
    pub pages: HashMap<String, String>,
}

// #[derive(Debug, Clone)]
// pub struct HTMLFile {
//     path: PathBuf,
//     content: String,
// }

// #[derive(Debug, Clone)]
// pub enum Style {
//     // todo: separate only into valid styles eg TextStyle
//     BackgroundColor(String), // todo: Color
//     Custom(String),          // custom css eg "border: 1px solid red" etc
// }

#[derive(Debug, Clone)]
pub struct State {
    pub page_buffers: HashMap<String, String>,
    variables_in_scope: Vec<Variable>,
    current_page_buffer: Option<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables_in_scope: Vec::new(),
            page_buffers: HashMap::new(),
            current_page_buffer: None, // TODO should be current_page, it's not the buffer.
        }
    }

    pub fn get_current_page_buffer(&mut self) -> ParseResult<&mut String> {
        if let Some(current_page) = self.current_page_buffer.clone() {
            if let Some(current_page_buffer) = self.page_buffers.get_mut(&current_page) {
                return Ok(current_page_buffer);
            }
        }
        // TODO return error
        panic!("oop");
    }

    // TODO extract this out into a multibuffer design pattern
    pub fn create_buffer(&mut self, key: String) -> ParseResult<()> {
        self.page_buffers.insert(key.clone(), String::new()); // FIXME check for collisions!
        self.current_page_buffer = Some(key);
        Ok(())
    }

    pub fn write_to_current_buffer(&mut self, string: &str) -> ParseResult<()> {
        Ok(self.get_current_page_buffer()?.push_str(string))
    }
}

/// run the interpreter over a series of nodes
pub fn run(nodes: &Vec<Node>, state: &mut State) -> ParseResult<()> {
    for node in nodes {
        match node {
            Node::Element(e) => {
                // println!("{}", e.ident);
                match e.ident.as_str() {
                    "page" => {
                        let path = get_required_path("path", &e.attributes)?;
                        write_page_buffer(path, state, &e.children)?;
                    }
                    "row" | "column" => {
                        state.write_to_current_buffer(&format!("<div class=\"{}\">", e.ident))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</div>")?;
                    }
                    "image" | "img" => {
                        state.write_to_current_buffer(&format!("<img />"))?;
                    }
                    _ => {
                        // panic!("");
                    }
                }
            }
            Node::Text(t) => {
                state.write_to_current_buffer(&t)?;
            }
            // _ => panic!("ERROR: unsupported function: {:?}", node),
            _ => (),
        }
    }

    Ok(())
}

pub fn get_required_variable(i: &str, attributes: &Vec<Attribute>) -> ParseResult<Variable> {
    for attribute in attributes {
        if let Attribute::Assignment { ident, variable } = attribute {
            if ident.eq(i) {
                return Ok(variable.clone());
            }
        }
    }
    panic!("attribute missing");
}

pub fn get_required_path(i: &str, attributes: &Vec<Attribute>) -> ParseResult<String> {
    match get_required_variable(i, attributes)? {
        Variable::RelativePath(s) => {
            return Ok(s.clone());
        }
        _ => {
            // TODO return Err 'wrong type'.
            panic!(format!("wrong type: {:?}", i));
        }
    }
}

/// returns a specific string from an attributes array or throws an error.
pub fn get_required_string(i: &str, attributes: &Vec<Attribute>) -> ParseResult<String> {
    match get_required_variable(i, attributes)? {
        Variable::QuotedString(s) => {
            return Ok(s.clone());
        }
        _ => {
            // TODO return Err 'wrong type'.
            panic!(format!("wrong type: {:?}", i));
        }
    }
}

use std::{collections::HashMap, path::PathBuf};
fn write_page_buffer(page: String, state: &mut State, nodes: &Vec<Node>) -> ParseResult<()> {
    state.create_buffer(page)?;
    state.write_to_current_buffer("<html>")?;
    run(nodes, state)?;
    state.write_to_current_buffer("</html>")
}

fn write_html_tag(ident: String, state: &mut State, nodes: &Vec<Node>) -> ParseResult<()> {
    Err(CassetteError::ParseError("hi".into()))
}

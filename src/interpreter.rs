use crate::error::*;
use crate::functions::*;
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
    // functions: Vec<Function>,
    variables_in_scope: Vec<Variable>,
    page_buffers: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables_in_scope: Vec::new(),
            page_buffers: HashMap::new(),
        }
    }
}

pub fn run(nodes: &Vec<Node>, state: &mut State) -> ParseResult<()> {
    // let mut functions = load_stdlib();
    // let mut site: Site = Site {
    //     pages: HashMap::new(),
    // };

    for node in nodes {
        match node {
            Node::Element(e) => {
                println!("{}", e.ident);
                match e.ident.as_str() {
                    "page" => {
                        // let mut buffer: Vec<u8> = Vec::new();
                        //let mut buffer = state.page_buffers.get_mut(&e.ident).unwrap();
                        write_page_buffer(String::from("index.html"), state, &e.children)?;
                    },
                    _ => (),
                }
            },
            _ => println!("ERROR: unsupported function: {:?}", node),
        }
    }

    println!("--{:?}", state);

    // extract_pages

    Ok(())
}

// use std::{path::PathBuf, io::Write, collections::HashMap};
// pub fn write_page_buffer<W: Write>(nodes: &Vec<Node>, writer: &mut W) -> ParseResult<()> {
//     Ok(())
// }

// use std::{path::PathBuf, io::Write, collections::HashMap};
// fn write_page_buffer<W: Write>(buffer: &mut W, state: &mut State, nodes: &Vec<Node>) -> ParseResult<()> {
//     buffer.write(&format!("</{}>", "self.ident").as_bytes())?;
//     Ok(())
// }

use std::{path::PathBuf, collections::HashMap};
fn write_page_buffer(page: String, state: &mut State, nodes: &Vec<Node>) -> ParseResult<()> {

    // buffer.write(&format!("</{}>", "self.ident").as_bytes())?;

    // state.page_buffers.get_mut(page).unwrap();
    println!("writing to {}", page);
    // state.page_buffers.get(page).unwrap();
    state.page_buffers.insert(page, String::from("<html>"));

    Ok(())
}

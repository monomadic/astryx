//! Astryx is a declarative, pure, expressive static web content compiler.
//! It draws inspiration from projects like elm-ui, haml and svelte, with the
//! type safety of rust and far, far easier setup and deployment.
//!
//! # Features
//! * declarative, type-checked, intuitive ui language
//! * clean separation of style, layout, and content
//! * single binary (makes CI/CD into github/gitlab pages very simple)
//! * zero boilerplate
//! * zero orphans
//! * smaller static sites than any other library, period.
//!

use error::{AstryxError, AstryxResult, AstryxErrorKind, Span};
use filesystem::get_folder_from_filename;
use interpreter::{self, html};
use parser::{self, Token};
use std::collections::HashMap;
//use html;

pub mod error;
mod filesystem;
// mod frontmatter;
// mod highlighter;
// mod html;
// mod interpolator;
// mod interpreter;
// mod imports;
// mod markdown;

// /// takes a path and returns a hashmap of rendered files. uses PWD for all files (paths are not relative to file)
// /// deprecated
// pub fn render_to_string_buffers<S: Into<String>>(
//     file_content: S,
// ) -> AstryxResult<HashMap<String, String>> {
//     std::fs::read_to_string(file_content.into())
//         .and_then(|file_content| parser::run(&file_content).map_err(|e| String::from(*e.into())))
//         .and_then(|ast| interpreter::run(&ast, None))
//         .and_then(|html_nodes| html::render_as_string(&html_nodes))
// }

fn convert_error(err: nom::Err) -> AstryxError {
    match err {
        _ => println!("{:?}", err)
    }
}

pub fn render<'a, S: Copy + Into<String>>(path: S) -> AstryxResult<HashMap<String, String>> {
    let file: &str = &std::fs::read_to_string(&path.into())?;

    parser::run(file)
        .map_err(|e| {
            convert_error(e);
            AstryxError{ kind: AstryxErrorKind::ParserError, pos: Span {
            line: e.unwrap_err(),
            position: 0,
            offset: 0,
        } }
        })
        .and_then(|(_span, ast)| {
            interpreter::run(&ast, get_folder_from_filename(&path.into()))
                .map_err(|_| AstryxError::new("file"))
        })
        .and_then(|html_nodes| {
            html::render_as_string(&html_nodes).map_err(|_| AstryxError::new("file"))
        })
}

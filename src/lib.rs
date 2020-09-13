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

use error::AstryxResult;
use filesystem::get_folder_from_filename;
use parser;
use std::collections::HashMap;

pub mod error;
mod filesystem;
mod frontmatter;
mod highlighter;
mod html;
// mod interpolator;
mod imports;
mod interpreter;
mod markdown;

/// takes a path and returns a hashmap of rendered files. uses PWD for all files (paths are not relative to file)
/// deprecated
pub fn render_to_string_buffers<S: Into<String>>(
    file_content: S,
) -> AstryxResult<HashMap<String, String>> {
    filesystem::read_file(file_content.into())
        .and_then(|file_content| parser::parse(&file_content).map_err(|e| e.into()))
        .and_then(|ast| interpreter::run(&ast, None))
        .and_then(|html_nodes| html::render_as_string(&html_nodes))
}

pub fn render<S: Copy + Into<String>>(path: S) -> AstryxResult<HashMap<String, String>> {
    filesystem::read_file(&path.into())
        .and_then(|file_content| parser::parse(&file_content).map_err(|e| e.into()))
        .and_then(|ast| interpreter::run(&ast, get_folder_from_filename(&path.into())))
        .and_then(|html_nodes| html::render_as_string(&html_nodes))
}

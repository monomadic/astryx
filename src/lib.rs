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
use parser;
use std::collections::HashMap;

pub mod error;
mod filesystem;
mod frontmatter;
mod highlighter;
mod html;
mod interpolator;
mod interpreter;
mod markdown;
mod modifiers;
// mod parser;
// mod variable;

/// takes a path and returns a hashmap of rendered files
pub fn render_to_string_buffers<S: Into<String>>(file: S) -> AstryxResult<HashMap<String, String>> {
    let tokens = parser::parse(&file.into())?;
    let nodes = interpreter::run(&tokens)?;

    html::render_as_string(&nodes)
}

// pub(crate) fn render<W: Write>(node: &Node<HTMLNode>, writer: &mut W) -> AstryxResult<()> {
// }

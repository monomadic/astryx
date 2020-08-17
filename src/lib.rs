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
mod parser;
mod variable;

/// takes a path and returns a hashmap of rendered files
// TODO: return a struct result? pages, files
pub fn render(file: &String) -> AstryxResult<HashMap<String, String>> {
    let state = &mut interpreter::State::new();
    let tokens = parser::parse(&file)?;
    let _ = interpreter::__run(&tokens, state)?;

    state.render_pages()
}

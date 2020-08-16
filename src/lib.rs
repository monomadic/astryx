//! Astryx is a declarative, pure, expressive static web content compiler.
//!
//! # Features
//! * Simple, easy-to-use, batteries-included API
//! * single binary (makes CI/CD into github/gitlab pages very simple)
//! * declarative, safe, correct language
//! * zero boilerplate
//! * (optionally) a single astryx program generates an entire site of static content and files including html, css, images to a degree, and scripts.

use error::AstryxResult;
use std::{collections::HashMap};

pub mod error;
mod filesystem;
mod frontmatter;
mod highlighter;
mod html;
mod interpolator;
mod interpreter;
mod markdown;
mod parser;
mod variable;
mod processors;

/// takes a path and returns a hashmap of rendered files
// TODO: return a struct result? pages, files
pub fn render(file: &String) -> AstryxResult<HashMap<String, String>> {
    let state = &mut interpreter::State::new();
    let tokens = parser::parse(&file)?;
    let _ = interpreter::__run(&tokens, state)?;

    state.render_pages()
}

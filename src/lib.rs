use error::AstryxResult;
use std::{collections::HashMap, path::PathBuf};

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

/// takes a path and returns a hashmap of rendered files
// TODO: return a struct result? pages, files
pub fn render(file: PathBuf) -> AstryxResult<HashMap<String, String>> {
    let state = &mut interpreter::State::new();
    let file = filesystem::read_file(file.clone())?;
    let tokens = parser::parse(&file)?;
    let _ = interpreter::__run(&tokens, state)?;

    state.render_pages()
}

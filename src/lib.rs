use error::AstryxResult;
use std::{collections::HashMap, path::PathBuf};

pub mod error;
pub mod filesystem;
mod frontmatter;
pub mod highlighter;
mod html;
mod interpolator;
pub mod interpreter;
mod markdown;
mod models;
pub mod parser;
mod variable;

pub fn render(file: PathBuf) -> AstryxResult<HashMap<String, String>> {
    let state = &mut interpreter::State::new();
    let file = filesystem::read_file(file.clone())?;
    let tokens = parser::parse(&file)?;
    let _ = interpreter::__run(&tokens, state)?;

    state.render_pages()
}

pub use error::{AstryxError, AstryxResult};

mod build;
mod init;
mod server;

pub use build::build;
pub use init::init_project;
use models::{Site, State};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

// Compiles program text into an output graph
pub fn parse_from_string(input: &str, path: &str, state: Option<State>) -> AstryxResult<Site> {
    let state = Rc::new(RefCell::new(state.unwrap_or(State::new())));

    let (rem, lines) =
        nom_indent::indent(input, path).map_err(|_| AstryxError::Generic("indent error".into()))?;

    // check for unexpected remaining content from indenter
    if !rem.len() == 0 {
        // fixme: return astryxerror
        panic!("non empty!");
    }

    parser::parse(lines)
        .and_then(|statements| interpreter::run(&statements, state))
        .map(Site::render)
}

// Compiles a source file into an output graph
pub fn parse_from_file<P: AsRef<Path>>(input: P, state: Option<State>) -> AstryxResult<Site> {
    let path: String = input.as_ref().to_str().unwrap().into();
    let input: String = std::fs::read_to_string(input).map_err(|e| AstryxError::IO(e))?;

    parse_from_string(&input, &path, state)
}

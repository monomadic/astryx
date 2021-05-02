// public api interface for astryx

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

// exports
pub use error::{AstryxError, AstryxResult};
pub use models::{Site, State};

// Compiles program text into an output graph
pub fn parse_from_string(input: &str, path: &str, state: Option<State>) -> AstryxResult<Site> {
    // if no initial state is given to us, pass new empty state.
    let state = Rc::new(RefCell::new(state.unwrap_or(State::new())));

    // try to read commands based on whitespace indentation
    let (rem, lines) =
        nom_indent::indent(input, path).map_err(|_| AstryxError::Generic("indent error".into()))?;

    // check for unexpected remaining content from indenter
    if !rem.len() == 0 {
        // fixme: return astryxerror
        panic!("non empty!");
    }

    // parse each line into statements, interpret them, and render a Site collection
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

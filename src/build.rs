use error::{AstryxError, AstryxResult};
use models::{Site, State};
use nom_indent;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Compiles an input file into an output graph
pub(crate) fn build<S: Into<String>, P: AsRef<Path>>(file: S, path: P) -> AstryxResult<()> {
    let state = Rc::new(RefCell::new(State::new()));
    let input: String = file.into();
    let path: String = path.as_ref().to_str().unwrap().into();

    parser::run(&input, &path)
        .map_err(AstryxError::from)
        .and_then(|nodes| interpreter::run(&nodes, state))
        .map(Site::render)
        .map(|site| site.write())
}

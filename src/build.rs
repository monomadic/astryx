use error::{AstryxError, AstryxResult};
use models::{Site, State};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

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

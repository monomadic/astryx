use error::{AstryxError, AstryxResult};
use models::{Site, State};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn build<'a>(file: &'a str, path: &str) -> AstryxResult<()> {
    let state = Rc::new(RefCell::new(State::new()));

    parser::run(file, path)
        .map_err(AstryxError::from)
        .and_then(|nodes| interpreter::run(&nodes, state))
        .map(Site::render)
        .map(|site| site.write())
}

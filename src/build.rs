use error::{AstryxError, AstryxResult};
use models::{Site, State};
use nom_indent;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Compiles an input file into an output graph
pub(crate) fn build<P: AsRef<Path>>(input: P) -> AstryxResult<()> {
    let file = std::fs::read_to_string(&input).map_err(|e| AstryxError::IO(e))?;
    let path: String = input.as_ref().to_str().unwrap().into();
    let state = Rc::new(RefCell::new(State::new()));

    // todo: fix this unwrap
    let (rem, lines) = nom_indent::indent(&file, &path).unwrap();
    // todo: check rem for errors

    parser::parse(lines)
        .map_err(AstryxError::from)
        .and_then(|statements| interpreter::run(&statements, state))
        .map(Site::render)
        .map(|site| site.write())
        .map_err(AstryxError::from)
    // nom_indent::indent(&file, &path)
    //     .map_err(AstryxError::from)
    //     .and_then(|(_, lines)| parser::parse(lines))
    //     .map(|nodes| interpreter::run(&nodes, state))
    //     .map(Site::render)
    //     .map(|site| site.write())
}

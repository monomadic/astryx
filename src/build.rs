use error::{AstryxError, AstryxResult};
use models::State;
use program::Project;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn build<'a>(file: &'a str) -> AstryxResult<()> {
    let state = Rc::new(RefCell::new(State::new()));

    parser::run(file)
        .map_err(AstryxError::from)
        .and_then(|nodes| interpreter::run(&nodes, state).map_err(AstryxError::from))
    // .map(program::render_project)
    // .and_then(write)
}

fn write<'a>(project: Project) -> AstryxResult<()> {
    // needs: mkdir ./build
    Ok(for (path, data) in project.pages {
        std::fs::write(format!("build/{}", route_to_path(&path)?), data)?;
    })
}

fn route_to_path<'a>(route: &str) -> AstryxResult<&str> {
    Ok(match route {
        "/" => "index.html",
        _ => "blah.html",
    })
}

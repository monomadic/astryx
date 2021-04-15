use error::AstryxResult;
use std::fs::copy;

/// set up a new project in the current directory
pub(crate) fn init_project<'a>() -> AstryxResult<()> {
    // copy site.astryx file
    let index = include_str!("../templates/site.astryx");

    // ? copy ./components/ or something?
    unimplemented!() //.map_err(|e| format!("error creating new project: {:?}", e))
}

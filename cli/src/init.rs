use astryx::{AstryxError, AstryxResult};
use std::fs;
use std::fs::copy;
use std::path::Path;

/// set up a new project in the current directory
pub fn init_project<'a>() -> AstryxResult<()> {
    // copy site.astryx file
    let index = include_str!("../templates/site.astryx");
    let path = Path::new("site.astryx");

    fs::write(path, index).map_err(AstryxError::from)
}

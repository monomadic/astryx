use astryx::{AstryxError, AstryxResult};
use std::{fs, path::Path};

/// set up a new project in the current directory
pub fn init_project<'a, P: AsRef<Path>>(path: P) -> AstryxResult<()> {
    let path = path.as_ref();

    // create output dir if it doesn't exist
    // todo: use std::fs::create_dir_all ?
    if !path.exists() {
        std::fs::create_dir(path)?;
    }

    // copy site.astryx file
    fs::write(
        path.join("site.astryx"),
        include_str!("../templates/site.astryx"),
    )
    .map_err(AstryxError::from)
}

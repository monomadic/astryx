use astryx::{AstryxError, AstryxResult};
use std::{fs, path::Path};

/// set up a new project in the current directory
pub fn init_project<'a, P: AsRef<Path>>(output: P) -> AstryxResult<()> {
    let output_path = output.as_ref();

    // create output dir if it doesn't exist
    if !output_path.exists() {
        std::fs::create_dir(output_path)?;
    }

    // copy site.astryx file
    fs::write(
        output_path.join("site.astryx"),
        include_str!("../templates/site.astryx"),
    )
    .map_err(AstryxError::from)
}

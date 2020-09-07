use astryx::error::*;
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn read_file(pathbuf: &std::path::PathBuf) -> AstryxResult<String> {
    let mut buffer = String::new();

    File::open(pathbuf)?
        .read_to_string(&mut buffer)?;

    Ok(buffer)
}

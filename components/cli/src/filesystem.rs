use astryx::error::*;
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn read_file(pathbuf: &std::path::PathBuf) -> AstryxResult<String> {
    let mut f = File::open(pathbuf.clone())
        .map_err(|_| AstryxError::new(&format!("error opening file: {:?}", pathbuf)))?;

    let mut buffer = String::new();

    f.read_to_string(&mut buffer)
        .map_err(|_| AstryxError::new("error reading file"))?;

    Ok(buffer)
}

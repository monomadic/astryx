use crate::{error::*};
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn read_file<S:ToString>(path: S) -> AstryxResult<String> {
    let mut buffer = String::new();
    let path = path.to_string();

    File::open(path.clone())
        .map_err(|_| AstryxError::new_from(AstryxErrorKind::FileNotFound(path.clone())))?
        .read_to_string(&mut buffer)
        .map_err(|_| AstryxError::new_from(AstryxErrorKind::CannotReadFile(path)))?;

    Ok(buffer)
}

use crate::error::*;
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn read_file(pathbuf: &str) -> AstryxResult<String> {
    let mut buffer = String::new();

    File::open(pathbuf).expect("file failed to read")
        .read_to_string(&mut buffer).expect("could not read to string"); // TODO remove unwrap

    Ok(buffer)
}

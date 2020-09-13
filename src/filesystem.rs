#![allow(dead_code)]

use crate::error::*;

use std::fs::File;
use std::io::prelude::*;
pub(crate) fn read_file<S: ToString>(path: S) -> AstryxResult<String> {
    let mut buffer = String::new();
    let path = path.to_string();

    File::open(path.clone())
        .map_err(|_| AstryxError::new_from(AstryxErrorKind::FileNotFound(path.clone())))?
        .read_to_string(&mut buffer)
        .map_err(|_| AstryxError::new_from(AstryxErrorKind::CannotReadFile(path)))?;

    Ok(buffer)
}

pub(crate) fn get_extension_from_filename(filename: &str) -> Option<&str> {
    use std::ffi::OsStr;
    use std::path::Path;

    Path::new(filename).extension().and_then(OsStr::to_str)
}

#[test]
fn test_get_extension_from_filename() {
    assert_eq!(get_extension_from_filename("abc.tar.gz"), Some("gz"));
    assert_eq!(get_extension_from_filename("abc..gz"), Some("gz"));
    assert_eq!(get_extension_from_filename("abc.gz"), Some("gz"));
}

pub(crate) fn get_folder_from_filename(filename: &str) -> Option<&str> {
    use std::path::Path;

    Path::new(filename).parent().and_then(|p| p.to_str())
}

#[test]
fn test_get_folder_from_filename() {
    assert_eq!(get_folder_from_filename("/a/b.zip"), Some("/a"));
    assert_eq!(get_folder_from_filename("../output.txt"), Some(".."));
    assert_eq!(get_folder_from_filename("examples/new/output.astryx"), Some("examples/new"));
}

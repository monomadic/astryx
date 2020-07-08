
pub(crate) fn read_file(pathbuf: std::path::PathBuf) -> ParseResult<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(pathbuf.clone()).map_err(|_| {
        AstryxError::ParseError("error opening file".into())
    })?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).map_err(|_| {
        AstryxError::ParseError("error reading file".into())
    })?;

    Ok(buffer)
}

use crate::error::*;
use crate::models::*;
use std::{collections::HashMap, path::PathBuf};

pub(crate) fn read_content_metadata(pattern: &str) -> ParseResult<Vec<TemplateFile>> {

    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs = glob::glob_with(&format!("./{}", pattern), options).map_err(|_| {
        AstryxError::ParseError("error globbing file".into())
    })?;

    for file in globs {
        // let path = file.map_err(|_| CassetteError::ParseError("file not found".into()))?;
        // let filename = file.pa
        let file_content = read_file(file.expect("file to unwrap"))?;

        files.push(TemplateFile {
            body: file_content.clone(),
            // nodes:
            // filename: format!("{:?}", file.unwrap()),
            // variables: HashMap::new(),
            metadata: crate::frontmatter::parse(&file_content).map_err(|_| {
                AstryxError::ParseError("error reading metadata".into())
            })?
        });
    };

    Ok(files)
}

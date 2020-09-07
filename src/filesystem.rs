use crate::{interpreter::Document, error::*};
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

// TODO rewrite as map
pub(crate) fn read_documents(pattern: &str) -> AstryxResult<Vec<Document>> {
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs = glob::glob_with(pattern, options)
        .map_err(|_| AstryxError::new("error globbing file"))?;

    for file in globs {
        // TODO wrap unwrap in error
        let filepath: String = file.expect("file to unwrap").as_os_str().to_str().unwrap().into();
        let file_content = read_file(filepath)?;

        let (yaml, markdown) = crate::frontmatter::parse(&file_content)
            .map_err(|_| AstryxError::new("error reading metadata"))?;

        files.push(Document {
            body: crate::markdown::parse(&markdown)?,
            metadata: yaml,
        });
    }

    Ok(files)
}

use crate::{variable::TemplateFile, error::*};

pub fn read_file(pathbuf: std::path::PathBuf) -> AstryxResult<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(pathbuf.clone()).map_err(|_| {
        AstryxError::new(&format!("error opening file: {:?}", pathbuf))
    })?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).map_err(|_| {
        AstryxError::new("error reading file")
    })?;

    Ok(buffer)
}


pub(crate) fn read_content_metadata(pattern: &str) -> AstryxResult<Vec<TemplateFile>> {

    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs = glob::glob_with(&format!("./{}", pattern), options).map_err(|_| {
        AstryxError::new("error globbing file")
    })?;

    for file in globs {
        // let path = file.map_err(|_| CassetteError::ParseError("file not found".into()))?;
        // let filename = file.pa
        let file_content = read_file(file.expect("file to unwrap"))?;

        let (yaml, markdown) = crate::frontmatter::parse(&file_content).map_err(|_| {
            AstryxError::new("error reading metadata")
        })?;

        files.push(TemplateFile {
            body: crate::markdown::parse(&markdown)?,
            // nodes:
            // filename: format!("{:?}", file.unwrap()),
            // variables: HashMap::new(),
            metadata: yaml
        });
    };

    Ok(files)
}

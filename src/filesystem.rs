
// pub(crate) fn read_file(pathbuf: std::path::PathBuf) -> crate::CassetteResult<String> {
//     use std::fs::File;
//     use std::io::prelude::*;

//     let mut f = File::open(pathbuf.clone()).map_err(|_| {
//         failure::format_err!(
//             "Could not open or read file: {}",
//             pathbuf.to_str().unwrap_or("")
//         )
//     })?;
//     let mut buffer = String::new();

//     f.read_to_string(&mut buffer)?;

//     Ok(buffer)
// }

use crate::error::*;
use crate::models::*;

pub(crate) fn read_content_metadata(pattern: &str) -> ParseResult<Vec<Metadata>> {
    use glob::glob_with;
    // use glob::MatchOptions;

    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for entry in glob_with(pattern, options).unwrap() {
        println!("---{:?}", entry);
    }
    Ok(Vec::new())
}

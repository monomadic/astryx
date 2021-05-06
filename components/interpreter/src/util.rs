use error::{AstryxError, AstryxErrorKind, AstryxResult, Location};
use glob::Paths;
use models::object::Object;
use parser::Span;
use rctree::Node;
use std::path::PathBuf;

pub(crate) fn span_to_location(span: Span) -> Location {
    Location {
        line: span.location_line(),
        column: span.get_column(),
        length: span.location_offset(),
        filename: span.extra.into(),
        context: String::from_utf8(span.get_line_beginning().into()).unwrap(),
    }
}

pub(crate) fn glob_files(s: &Span, pwd: Option<Object>) -> AstryxResult<Object> {
    let pattern = match pwd {
        Some(pwd) => format!("{}/{}", pwd.to_string(), s),
        None => s.to_string(),
    };

    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs: Paths = glob::glob_with(&pattern, options).map_err(|e| {
        AstryxError::LocatedError(span_to_location(*s), AstryxErrorKind::Unexpected)
    })?;

    for file in globs {
        // TODO wrap unwrap in error
        let path = file.expect("file to unwrap");
        let filepath: String = path.as_os_str().to_str().unwrap().into();

        files.push(Node::new(Object::Path(filepath)));
    }

    Ok(Object::Array(files))
}

// pub(crate) fn import_files<'a>(s: &Span<'a>) -> AstryxResult<Object> {
//     let options = glob::MatchOptions {
//         case_sensitive: false,
//         require_literal_separator: false,
//         require_literal_leading_dot: false,
//     };
//
//     let mut files = Vec::new();
//     let globs: Paths = glob::glob_with(&s.to_string(), options).map_err(|e| {
//         AstryxError::LocatedError(span_to_location(*s), AstryxErrorKind::Unexpected)
//     })?;
//
//     for file in globs {
//         // TODO wrap unwrap in error
//         let path = file.expect("file to unwrap");
//         let filepath: String = path.as_os_str().to_str().unwrap().into();
//         let file_content = std::fs::read_to_string(filepath).unwrap();
//
//         files.push(Node::new(Object::String(file_content)));
//     }
//
//     Ok(Object::Array(files))
// }
//
// pub(crate) fn import_file(s: &Span) -> AstryxResult<Object> {
//     std::fs::read_to_string(s.fragment().to_string())
//         .map(Object::String)
//         .map_err(|e| AstryxError::LocatedError(span_to_location(*s), AstryxErrorKind::Unexpected))
// }

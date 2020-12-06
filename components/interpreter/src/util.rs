use crate::{models::Object, InterpreterError, InterpreterErrorKind, InterpreterResult};
use glob::Paths;
use parser::Span;
use rctree::Node;

pub(crate) fn import_files<'a>(s: &Span<'a>) -> InterpreterResult<Object> {
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs: Paths = glob::glob_with(&s.to_string(), options).map_err(|_| InterpreterError {
        kind: InterpreterErrorKind::IOError,
        location: Some((*s).into()),
    })?;

    for file in globs {
        // TODO wrap unwrap in error
        let path = file.expect("file to unwrap");
        let filepath: String = path.as_os_str().to_str().unwrap().into();
        let file_content = std::fs::read_to_string(filepath).unwrap();

        files.push(Node::new(Object::String(file_content)));
    }

    Ok(Object::Array(files))
}

pub(crate) fn import_file<'a>(s: &Span<'a>) -> InterpreterResult<Object> {
    std::fs::read_to_string(s.fragment().to_string())
        .map(Object::String)
        .map_err(|_| InterpreterError {
            kind: InterpreterErrorKind::IOError,
            location: Some((*s).into()),
        })
}

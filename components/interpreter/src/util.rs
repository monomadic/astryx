use error::{AstryxError, AstryxResult, Location};
use glob::Paths;
use models::object::Object;
use parser::Span;
use rctree::Node;

pub(crate) fn span_to_location(span: Span) -> Location {
    Location {
        line: span.location_line(),
        column: span.get_column(),
        length: span.location_offset(),
        filename: span.extra.into(),
        context: String::from_utf8(span.get_line_beginning().into()).unwrap(),
    }
}

pub(crate) fn glob_files<S: ToString>(pattern: S) -> AstryxResult<Object> {
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs: Paths = glob::glob_with(&pattern.to_string(), options)
        .map_err(|_| AstryxError::Generic("glob error".into()))?;

    for file in globs {
        // TODO wrap unwrap in error
        let path = file.expect("file to unwrap");
        let filepath: String = path.as_os_str().to_str().unwrap().into();

        files.push(Node::new(Object::Path(filepath)));
    }

    Ok(Object::Array(files))
}

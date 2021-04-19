use crate::Span;
use error::Location;

pub(crate) fn span_to_location(span: Span) -> Location {
    Location {
        line: span.location_line(),
        column: span.get_column(),
        length: span.location_offset(),
        filename: span.extra.into(),
        context: String::from_utf8(span.get_line_beginning().into()).unwrap(),
    }
}

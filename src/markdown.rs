use crate::error::*;
use markdown;

pub fn parse(i: &str) -> ParseResult<String> {
    // TODO use a stricter lib that will throw errors, or
    // write one that returns a syntax tree of nodes
    Ok(markdown::to_html(i))
}

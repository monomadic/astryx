use crate::error::*;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::io::{stdout};

pub fn parse(i: &str) -> AstryxResult<String> {
    // TODO use a stricter lib that will throw errors, or
    // write one that returns a syntax tree of nodes
    let parser = Parser::new_ext(i, Options::empty());

    let mut buf: Vec<u8> = Vec::new();
    html::write_html(&mut buf, parser).unwrap();
    Ok(String::from_utf8(buf).unwrap())
}

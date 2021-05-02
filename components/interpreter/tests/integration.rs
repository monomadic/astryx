use error::AstryxError;
use models::{Site, State};
use parser;
use std::cell::RefCell;
use std::rc::Rc;

fn eval(input: &str) {
    let state = Rc::new(RefCell::new(State::new()));

    let site = nom_indent::indent(input, "<test>")
        .map_err(|_e| AstryxError::Generic("blah".into()))
        .and_then(|(_, lines)| parser::parse(lines))
        .and_then(|statements| interpreter::run(&statements, state))
        .map(Site::render)
        .unwrap();

    println!("{:?}", site);
}

#[test]
fn test_string_literal() {
    eval(r#""a string""#);
}

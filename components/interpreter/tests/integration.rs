#![deny(warnings)]

use interpreter::*;
use parser::*;
use std::cell::RefCell;
use std::rc::Rc;

#[must_use]
struct StatementTestRunner {
    state: State,
}

impl StatementTestRunner {
    fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    fn eval(&mut self, statement: &str) -> &mut Self {
        let statements = parser::run(&statement)
            .map(|r| interpreter::run(&r, Rc::new(RefCell::new(self.state))).unwrap())
            .unwrap();
        self
    }

    fn assert(&self, object: Object) -> &Self {
        self
    }
}

#[test]
fn test_string_literal() {
    StatementTestRunner::new()
        .eval(r#""a string""#)
        .assert(Object::String(String::from("a string")));
}

#[test]
fn test_string_expression() {
    assert_eq!(interpreter::eval(statement, state))
}

fn assert_run(test: String, state: State, result: &str) {
    let state = Rc::new(RefCell::new(state));

    let statements = parser::run(&test)
        .map(|r| interpreter::run(&r, state).unwrap())
        .unwrap();

    assert_eq!(
        statements.iter().map(|s| s.inspect()).collect::<String>(),
        result
    );
}

#[test]
fn test_parser() {
    assert!(interpreter::run(&Vec::new(), Rc::new(RefCell::new(State::new()))).is_ok());

    assert_run("post.title".into(), State::new(), "post.title");
    assert_run("post.markdown()".into(), State::new(), "post.markdown()");
}

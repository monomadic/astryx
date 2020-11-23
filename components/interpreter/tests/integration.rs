use interpreter;
use interpreter::State;
use parser;
use std::cell::RefCell;
use std::rc::Rc;

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

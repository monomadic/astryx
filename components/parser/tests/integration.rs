use parser::*;

fn assert_run(test: &str, result: &str) {
    assert_eq!(
        run(test)
            .unwrap()
            .iter()
            .map(|r| r.borrow().inspect())
            .collect::<String>(),
        result
    );
}

#[test]
fn test_parser() {
    assert!(run("").is_ok());
    assert!(run("\n").is_ok());

    assert_run("post.title", "post.title");
    assert_run("post.markdown()", "post.markdown()");
}

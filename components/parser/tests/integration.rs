use nom_locate::position;
use parser::*;

fn assert_statement_eq(test: &str, result: &str) {
    assert_eq!(
        parser::statement::statement(Span::from(test))
            .unwrap()
            .1
            .inspect(),
        result.to_string()
    );
}

#[test]
fn test_parser() {
    assert_statement_eq("post.title", "post.title");
    assert_statement_eq("post.markdown()", "post.markdown()");
}

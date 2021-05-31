fn assert_statement_eq(test: &str, result: &str) {
    assert_eq!(
        parser::statement(test).unwrap().1.to_string(),
        result.to_string()
    );
}

#[test]
fn test_parser() {
    assert_statement_eq(
        "post.title",
        "Expression(Expression::Index(Reference(post), Reference(title)))",
    );
    assert_statement_eq(
        "post.markdown()",
        "Expression(Expression::Index(Reference(post), FunctionCall(Reference(markdown))))",
    );
}

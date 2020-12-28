use nom_locate::position;
use parser::*;

fn assert_run(test: &str, result: &str) {
    assert_eq!(
        run(test, "")
            .unwrap()
            .iter()
            .map(|r| r.borrow().inspect())
            .collect::<String>(),
        result
    );
}

#[test]
fn test_parser() {
    assert!(run("", "").is_ok());
    assert!(run("\n", "").is_ok());
    assert!(run("print()", "").is_ok());
    assert!(run("print()\n", "").is_ok());

    assert_run("post.title", "post.title");
    assert_run("post.markdown()", "post.markdown()");
}

#[test]
fn test_error_position() {
    fn assert_error_position(i: &str, line: u32, column: usize) {
        let input = Span::new_extra(i, "");
        let err = parser::parse(input).unwrap_err();

        if let nom::Err::Error(e) = err {
            let (s, pos) = position::<Span<'_>, ParserError<Span<'_>>>(e.pos).unwrap();
            assert_eq!(pos.get_column(), column);
            assert_eq!(pos.location_line(), line);
        // assert_eq!(pos.location_offset(), line);
        } else {
            panic!("error not returned!");
        }
    }

    assert_error_position("!", 1, 1);
    assert_error_position("print()!", 1, 8);
    assert_error_position("\n!", 2, 1);
}

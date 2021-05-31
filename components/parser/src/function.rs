use crate::{
    errorold::ParserErrorKind, statement::expression, Expression, FunctionCall,
    FunctionCallArguments, ParserError, Span,
};
use nom::character::complete::alphanumeric1;
use nom::multi::separated_list0;
use nom::{
    branch::alt,
    character::complete::{alpha1, char, multispace0, space0},
    combinator::{cut, map},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn function_call_named_argument(i: Span) -> IResult<Span, (Span, Expression), ParserError<Span>> {
    tuple((
        alpha1,
        terminated(multispace0, char(':')),
        space0,
        cut(expression),
    ))(i)
    .map(|(r, (ident, _, _, value))| (r, (ident, value)))
    .map_err(|e: nom::Err<_>| {
        e.map(|e| ParserError {
            context: i,
            kind: ParserErrorKind::ExpectedValue,
            pos: e.context,
        })
    })
}

fn function_call_named_arguments(
    i: Span,
) -> IResult<Span, Vec<(Span, Expression)>, ParserError<Span>> {
    separated_list1(
        tuple((space0, char(','), space0)),
        function_call_named_argument,
    )(i)
}

fn function_call_unnamed_arguments(i: Span) -> IResult<Span, Vec<Expression>, ParserError<Span>> {
    separated_list1(tuple((space0, char(','), space0)), expression)(i)
}

pub(crate) fn function_call(i: Span) -> IResult<Span, FunctionCall, ParserError<Span>> {
    alt((bracketed_function_call, hashmap_function_call))(i)
}

pub fn bracketed_function_call(i: Span) -> IResult<Span, FunctionCall, ParserError<Span>> {
    tuple((alpha1, char('('), function_call_arguments, cut(char(')'))))(i).map(
        |(r, (ident, _, arguments, _))| {
            (
                r,
                FunctionCall {
                    ident: Box::new(Expression::Reference(ident)),
                    arguments,
                },
            )
        },
    )
}

pub fn hashmap_function_call(i: Span) -> IResult<Span, FunctionCall, ParserError<Span>> {
    tuple((
        alphanumeric1,
        space0,
        char('{'),
        separated_list0(
            tuple((space0, char(','), space0)),
            function_call_named_argument,
        ),
        space0,
        cut(char('}')),
    ))(i)
    .map(|(r, (ident, _, _, args, _, _))| {
        (
            r,
            FunctionCall {
                ident: Box::new(Expression::Reference(ident)),
                arguments: FunctionCallArguments::Named(args),
            },
        )
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn expect_hashmap_function_call(tests: Vec<(&str, &str)>) {
        for (input, expected) in tests {
            match hashmap_function_call(Span::from(input)) {
                Ok(obj) => {
                    assert!(obj.0.is_empty());
                    assert_eq!(expected, obj.1.to_string(), "for `{}`", input);
                }
                Err(err) => {
                    panic!(
                        "expected `{}`, but got error=`{}` for `{}`",
                        expected, err, input
                    );
                }
            }
        }
    }

    #[test]
    fn test_hashmap_function_call() {
        expect_hashmap_function_call(vec![
            ("h1 {}", "Reference(h1)"),
            ("h1 { }", "Reference(h1)"),
            ("h1{}", "Reference(h1)"),
            ("h1 {a:1}", "Reference(h1)"),
        ]);
    }
}

fn function_call_arguments(i: Span) -> IResult<Span, FunctionCallArguments, ParserError<Span>> {
    alt((
        map(function_call_named_arguments, |v| {
            FunctionCallArguments::Named(v)
        }),
        map(function_call_unnamed_arguments, |v| {
            FunctionCallArguments::Unnamed(v)
        }),
        map(space0, |_| FunctionCallArguments::None),
    ))(i)
}

#[cfg(test)]
mod test_2 {
    use super::*;

    fn expect_function_call_arguments(tests: Vec<(&str, &str)>) {
        for (input, expected) in tests {
            match function_call_arguments(Span::from(input)) {
                Ok(obj) => {
                    assert!(obj.0.is_empty());
                    assert_eq!(expected, obj.1.to_string(), "for `{}`", input);
                }
                Err(err) => {
                    panic!(
                        "expected `{}`, but got error=`{}` for `{}`",
                        expected, err, input
                    );
                }
            }
        }
    }

    #[test]
    fn test_function_call_arguments() {
        expect_function_call_arguments(vec![
            ("", "None"),
            ("5", "Unnamed"),
            ("arg: 0", "Named"),
            ("a:a,b:b", "Named"),
        ]);
    }

    #[test]
    fn test_function_call() {
        assert!(function_call(Span::new_extra("g()", "")).is_ok());

        assert_eq!(
            &function_call(Span::new_extra("print(text: \"hello\")", ""))
                .unwrap()
                .1
                .inspect(),
            "print(text: \"hello\")"
        );

        assert_eq!(
            &function_call(Span::new_extra(
                "print(text: \"hello\", another: \"hi\")",
                ""
            ))
            .unwrap()
            .1
            .inspect(),
            "print(text: \"hello\", another: \"hi\")"
        );

        // check ident Span
        // let f: FunctionCall = function_call(Span::new("function()")).unwrap().1;
        // assert_eq!(f.ident.to_string(), "function");
        // assert_eq!(f.ident.location_line(), 1);
        // assert_eq!(f.ident.location_offset(), 0);
        // assert_eq!(f.ident.get_column(), 1);

        // check no-match with error
        let e = function_call(Span::new_extra("g", ""));
        match e {
            Err(nom::Err::Error(_)) => (),
            _ => panic!("expected Error, got {:?}", e),
        };

        // check partial match with fail
        let e = function_call(Span::new_extra("g(1)", ""));
        match e {
            Err(nom::Err::Failure(_)) => (),
            _ => panic!("expected Failure, got {:?}", e),
        };
    }
}

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

pub(crate) fn function_call(i: Span) -> IResult<Span, FunctionCall, ParserError<Span>> {
    alt((bracketed_function_call, hashmap_function_call))(i)
}

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
mod test {
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

    /// generic assertion of parser/combinator results
    fn assert_result<S: ToString>(args: (IResult<Span, S, ParserError<Span>>, &str, &str)) {
        let (result, input, expected) = args;

        match result {
            Ok((rem, obj)) => {
                assert!(rem.is_empty());
                assert_eq!(expected, obj.to_string(), "for `{}`", input);
            }
            Err(err) => {
                panic!(
                    "expected `{}`, but got error=`{}` for `{}`",
                    expected, err, input
                );
            }
        }
    }

    #[test]
    fn test_function_call() {
        let _ = vec![
            ("a", "Reference(a)"),
            ("print()", "Reference()"),
            ("h1 {}", "Reference(h1)"),
            ("h1 { }", "Reference(h1)"),
            ("h1{}", "Reference(h1)"),
            ("h1 {a:1}", "Reference(h1)"),
        ]
        .into_iter()
        .map(|(input, expected)| (function_call(Span::from(input)), input, expected))
        .map(assert_result)
        .collect::<Vec<_>>();
    }

    #[test]
    fn test_hashmap_function_call() {
        let _ = vec![
            ("h1 {}", "Reference(h1)"),
            ("h1 { }", "Reference(h1)"),
            ("h1{}", "Reference(h1)"),
            ("h1 {a:1}", "Reference(h1)"),
        ]
        .into_iter()
        .map(|(input, expected)| (hashmap_function_call(Span::from(input)), input, expected))
        .map(assert_result);
    }
}

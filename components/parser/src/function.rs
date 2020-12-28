use crate::{
    error::ParserErrorKind, statement::expression, Expression, FunctionCall, ParserError, Span,
};
use nom::{
    character::complete::{alpha1, multispace0},
    character::complete::{char, space0},
    combinator::cut,
    multi::separated_list0,
    sequence::{terminated, tuple},
    IResult,
};

fn function_call_argument<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, (Span<'a>, Expression<'a>), ParserError<Span<'a>>> {
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

fn function_call_arguments<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<(Span<'a>, Expression<'a>)>, ParserError<Span<'a>>> {
    // many0(function_call_argument)(i)

    separated_list0(tuple((space0, char(','), space0)), function_call_argument)(i)
    // .map_err(|e:nom::Err<ParserError<_>>| {
    //     e.map(|s| ParserError {
    //         context: i,
    //         kind: ParserErrorKind::UnexpectedToken("g".into()),
    //         pos: s.context.into(),
    //     })
    // })
}

pub(crate) fn function_call<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, FunctionCall<'a>, ParserError<Span<'a>>> {
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
    // .map_err(|e| {
    //     e.map(|s| ParserError {
    //         context: i,
    //         kind: ParserErrorKind::UnexpectedToken("blah".into()),
    //         pos: s.context.into(),
    //     })
    // })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_function_call_arguments() {
        assert_eq!(
            function_call_arguments(Span::new_extra("a:a,b:b", ""))
                .unwrap()
                .1
                .len(),
            2
        );
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

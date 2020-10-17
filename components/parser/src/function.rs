use crate::{error::ParserErrorKind, FunctionCall, ParserError, Span, Variable, variable::variable};
use nom::{
    character::complete::{space0, char},
    character::complete::{alpha1, multispace0},
    combinator::cut,
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

fn function_call_argument<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, (Span<'a>, Variable<'a>), ParserError<Span<'a>>> {
    tuple((alpha1, terminated(multispace0, char(':')), space0, cut(variable)))(i)
        .map(|(r, (ident, _, _, value))| (r, (ident, value)))
        .map_err(|e:nom::Err<_>| {
            e.map(|e| ParserError {
                context: i,
                kind: ParserErrorKind::ExpectedValue,
                pos: e.context,
            })
        })
}

fn function_call_arguments<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<(Span<'a>, Variable<'a>)>, ParserError<Span<'a>>> {
    many0(function_call_argument)(i)
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
    tuple((
        alpha1,
        char('('),
        function_call_arguments,
        cut(char(')')),
    ))(i)
    .map(|(r, (ident, _, arguments, _))| (r, FunctionCall { ident, arguments }))
    // .map_err(|e| {
    //     e.map(|s| ParserError {
    //         context: i,
    //         kind: ParserErrorKind::UnexpectedToken("blah".into()),
    //         pos: s.context.into(),
    //     })
    // })
}

#[test]
fn test_function_call() {
    assert!(function_call(Span::new("g()")).is_ok());

    // check ident Span
    let f: FunctionCall = function_call(Span::new("function()")).unwrap().1;
    assert_eq!(f.ident.to_string(), "function");
    assert_eq!(f.ident.location_line(), 1);
    assert_eq!(f.ident.location_offset(), 0);
    assert_eq!(f.ident.get_column(), 1);

    // check no-match with error
    let e = function_call(Span::new("g"));
    match e {
        Err(nom::Err::Error(_)) => (),
        _ => panic!("expected Error, got {:?}", e),
    };

    // check partial match with fail
    let e = function_call(Span::new("g(1)"));
    match e {
        Err(nom::Err::Failure(_)) => (),
        _ => panic!("expected Failure, got {:?}", e),
    };
}

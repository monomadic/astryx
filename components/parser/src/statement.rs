use crate::{
    element::element,
    error::ParserErrorKind,
    function::function_call,
    models::Statement,
    text::piped_string,
    variable::{glob_pattern, literal, relative_path},
    Expression, ParserError, Route, Span,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, space0, space1},
    combinator::{all_consuming, cut, map},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

// fn array<'a>(i: Span) -> IResult<Span, Span, ParserError<Span>> {
//     // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
//     tuple((char('['), alpha1, char(']')))(i)
//         .map(|(r, (_, ident, _))| (r, ident))
//         .map_err(|e| {
//             e.map(|(span, _kind)| ParserError {
//                 context: span,
//                 kind: ParserErrorKind::SyntaxError,
//                 pos: span.into(),
//             })
//         })
// }

// #[test]
// fn test_array() {
//     assert!(array(Span::new("[g]")).is_ok());
// }

pub(crate) fn statement<'a>(i: Span<'a>) -> IResult<Span, Statement<'a>, ParserError<Span<'a>>> {
    all_consuming(alt((
        // map(function_call, |f| Statement::FunctionCall(f)),
        map(comment, |s| Statement::Comment(s)),
        map(for_loop, |(ident, expr)| Statement::ForLoop { ident, expr }),
        map(binding, |(ident, expr)| Statement::Binding(ident, expr)),
        map(expression, |e| Statement::Expression(e)),
        map(route, |r| Statement::Route(r)),
        map(element, |e| Statement::Element(e)),
        map(piped_string, |e| Statement::Text(e)),
        // map(alpha1, |e| Statement::Element(e)),
        // return_statement
    )))(i)
    .map_err(|e| {
        e.map(|s| ParserError {
            context: i, // we need to reset the context to the whole line
            kind: s.kind,
            pos: s.pos,
        })
    })
}

fn for_loop<'a>(i: Span<'a>) -> IResult<Span, (Span<'a>, Expression<'a>), ParserError<Span<'a>>> {
    tuple((
        tag("for"),
        space1,
        alphanumeric1,
        terminated(space1, tag("in")),
        space1,
        expression,
    ))(i)
    .map(|(r, (_, _, ident, _, _, expr))| (r, (ident, expr)))
}

#[derive(Debug, Clone)]
pub struct ForLoop<'a> {
    pub index: Span<'a>,
    pub iterable: Expression<'a>,
}

fn route<'a>(i: Span<'a>) -> IResult<Span<'a>, Route<'a>, ParserError<Span<'a>>> {
    tuple((tag("@"), alphanumeric1, space0, many0(attribute_assignment)))(i)
        .map(|(r, (_, ident, _, attributes))| (r, Route { ident, attributes }))
        .map_err(|e: nom::Err<_>| {
            e.map(|e: ParserError<Span<'a>>| ParserError {
                context: e.context,
                kind: ParserErrorKind::SyntaxError,
                pos: i.into(),
            })
        })
}

// todo: replace
fn attribute_assignment<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, (Span<'a>, Expression), ParserError<Span<'a>>> {
    nom::sequence::tuple((
        multispace0,
        alpha1,
        terminated(multispace0, char('=')),
        space0,
        cut(expression),
    ))(i)
    .map(|(r, (_, ident, _, _, value))| (r, (ident, value)))
}

pub(crate) fn expression<'a>(i: Span<'a>) -> IResult<Span, Expression<'a>, ParserError<Span<'a>>> {
    alt((
        map(index, |(index, expr)| {
            Expression::Index(Box::new(index), Box::new(expr))
        }),
        map(relative_path, |s| Expression::RelativePath(s)),
        map(glob_pattern, |s| Expression::GlobPattern(s)),
        map(function_call, |f| Expression::FunctionCall(f)),
        map(literal, |v| Expression::Literal(v)),
        map(alphanumeric1, |s| Expression::Reference(s)),
    ))(i)
}

fn index<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, (Expression<'a>, Expression<'a>), ParserError<Span<'a>>> {
    tuple((index_expression, tag("."), expression))(i)
        .map(|(r, (index, _, expr))| (r, (index, expr)))
    // separated_list(tag("."), expression)(i)
    // tag("--")(i).map(|(r, _)| (Span::new(""), r))
}

fn index_expression<'a>(i: Span<'a>) -> IResult<Span, Expression<'a>, ParserError<Span<'a>>> {
    alt((
        //map(relative_path, |s| Expression::RelativePath(s)),
        map(glob_pattern, |s| Expression::GlobPattern(s)),
        map(function_call, |f| Expression::FunctionCall(f)),
        map(literal, |v| Expression::Literal(v)),
        map(alphanumeric1, |s| Expression::Reference(s)),
    ))(i)
    .map_err(|e| {
        nom::Err::Error(ParserError {
            kind: ParserErrorKind::Unexpected,
            pos: i,
            context: i,
        })
    })
}

fn comment<'a>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>, ParserError<Span<'a>>> {
    tag("--")(i).map(|(r, _)| (Span::new_extra("", ""), r)) // FXIME
                                                            // .map_err(|e| {
                                                            //     e.map(|(span, _kind)| ParserError {
                                                            //         context: span,
                                                            //         kind: ParserErrorKind::SyntaxError,
                                                            //         pos: span.into(),
                                                            //     })
                                                            // })
}

fn binding<'a>(i: Span<'a>) -> IResult<Span, (Span<'a>, Expression<'a>), ParserError<Span<'a>>> {
    tuple((
        tag("let"),
        space1,
        alphanumeric1,
        terminated(space0, tag("=")),
        space0,
        expression,
    ))(i)
    .map(|(r, (_, _, ident, _, _, expr))| (r, (ident, expr)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expression() {
        assert!(expression(Span::new_extra("./posts/*.md", "")).is_ok());
    }

    #[test]
    fn test_for_loop() {
        // println!("{:?}", for_loop(Span::new("for x in ./posts/*.md")));
        assert!(for_loop(Span::new_extra("for x in ./posts/*.md", "")).is_ok());
    }

    #[test]
    fn test_index() {
        assert!(index(Span::new_extra("test", "")).is_err());
        assert!(index(Span::new_extra("test.blah", "")).is_ok());
        assert!(index(Span::new_extra("test.log()", "")).is_ok());
    }

    #[test]
    fn test_binding() {
        assert!(binding(Span::new_extra("let a=5", "")).is_ok());
        // assert_eq!(binding(Span::new("let a=5")).unwrap().0.fragment().to_string(), "a");
        assert!(binding(Span::new_extra("let a = 5", "")).is_ok());
        assert!(binding(Span::new_extra("let print = print()", "")).is_ok());
        assert!(binding(Span::new_extra("let print = fn print()", "")).is_ok());
        assert!(binding(Span::new_extra("g()", "")).is_err());
    }

    #[test]
    fn test_route() {
        assert!(route(Span::new_extra("", "")).is_err());
        assert!(route(Span::new_extra("@", "")).is_err());
        assert!(route(Span::new_extra("@route", "")).is_ok());
        assert_eq!(
            route(Span::new_extra("@route", ""))
                .unwrap()
                .1
                .ident
                .to_string(),
            "route"
        );
        assert!(route(Span::new_extra("@route a=5", "")).is_ok());
    }

    #[test]
    fn test_statement() {
        assert!(statement(Span::new_extra("", "")).is_err()); // do not allow blank lines to slip through
        assert!(statement(Span::new_extra("g()", "")).is_ok());
        assert!(statement(Span::new_extra("for x in ./posts/*.md", "")).is_ok());
    }
}

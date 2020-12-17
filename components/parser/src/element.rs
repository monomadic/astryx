use crate::{
    error::ParserErrorKind, statement::expression, Element, Expression, ParserError, Route, Span,
};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    character::complete::{char, space0},
    combinator::cut,
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

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

pub(crate) fn element<'a>(i: Span<'a>) -> IResult<Span<'a>, Element<'a>, ParserError<Span<'a>>> {
    tuple((tag("%"), alphanumeric1, space0, many0(attribute_assignment)))(i)
        .map(|(r, (_, ident, _, attributes))| (r, Element { ident, attributes }))
        .map_err(|e: nom::Err<_>| {
            e.map(|e: ParserError<Span<'a>>| ParserError {
                context: e.context,
                kind: ParserErrorKind::SyntaxError,
                pos: i.into(),
            })
        })
}

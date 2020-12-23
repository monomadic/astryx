use crate::{
    error::ParserErrorKind, statement::expression, text::tokenised_string, Element, Expression,
    ParserError, Span,
};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    character::complete::{char, space0},
    combinator::{cut, opt},
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn attributes_braced<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<(Span<'a>, Expression)>, ParserError<Span<'a>>> {
    preceded(
        char('{'),
        cut(terminated(
            separated_list0(char(','), attribute_assignment),
            char('}'),
        )),
    )(i)
}

fn attribute_assignment<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, (Span<'a>, Expression), ParserError<Span<'a>>> {
    nom::sequence::tuple((
        multispace0,
        alpha1,
        terminated(multispace0, char(':')),
        space0,
        cut(expression),
        space0,
    ))(i)
    .map(|(r, (_, ident, _, _, value, _))| (r, (ident, value)))
}

pub(crate) fn element<'a>(i: Span<'a>) -> IResult<Span<'a>, Element<'a>, ParserError<Span<'a>>> {
    tuple((
        tag("%"),
        alphanumeric1,
        space0,
        opt(attributes_braced),
        space0,
        opt(tokenised_string),
    ))(i)
    .map(|(r, (_, ident, _, attributes, _, text))| {
        (
            r,
            Element {
                ident,
                attributes: attributes.unwrap_or(vec![]),
                text,
            },
        )
    })
    .map_err(|e: nom::Err<_>| {
        e.map(|e: ParserError<Span<'a>>| ParserError {
            context: e.context,
            kind: ParserErrorKind::SyntaxError,
            pos: i.into(),
        })
    })
}

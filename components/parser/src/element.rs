use crate::{error::ParserErrorKind, Element, ParserError, Span, variable::variable, Variable};
use nom::{
    character::complete::{space0, char},
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{cut, opt},
    multi::many0,
    sequence::{terminated, tuple},
    IResult, bytes::complete::tag,
};

fn attribute_assignment<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, (Span<'a>, Variable), ParserError<Span<'a>>> {
    nom::sequence::tuple((
        alpha1,
        terminated(multispace0, char('=')),
        space0,
        cut(variable),
    ))(i)
    // .map_err(|e: nom::Err<(_, _)>| {
    //     e.map(|(span, _kind)| ParserError {
    //         context: span,
    //         kind: ParserErrorKind::SyntaxError,
    //         pos: span.into(),
    //     })
    // })
    .map(|(r, (ident, _, _, value))| (r, (ident, value)))

}

// pub(crate) fn attribute<'a>(i: Span<'a>) -> IResult<Span<'a>, Vec<Span<'a>>, ParserError<Span<'a>>> {
//     // alt((
//     //     // map(decorator, |d| Attribute::Decorator(d)),
//     //     // map(dotted_symbol, |s| Attribute::Class(s)),
//     //     // attribute_assignment,
//     //     // map(symbolic1, |s| Attribute::Symbol(String::from(s))),
//     // ))(i)
//     many0(attribute_assignment)(i)
// }

pub(crate) fn element<'a>(i: Span<'a>) -> IResult<Span<'a>, Element<'a>, ParserError<Span<'a>>> {
    tuple((tag("%"), alphanumeric1, opt(char(' ')), many0(attribute_assignment)))(i)
        .map(|(r, (_, ident, _, attributes))| (r, Element { ident, attributes }))
        .map_err(|e: nom::Err<_>| {
            e.map(|e: ParserError<Span<'a>>| ParserError {
                context: e.context,
                kind: ParserErrorKind::SyntaxError,
                pos: i.into(),
            })
        })
}

use crate::{error::ParserErrorKind, variable::variable, ParserError, Span, StringToken, Variable};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    character::complete::multispace0,
    combinator::map,
    IResult,
};

pub(crate) fn piped_string<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<StringToken>, ParserError<Span<'a>>> {
    nom::sequence::tuple((tag("| "), tokenised_string))(i)
        .map(|(r, (_, value))| (r, value))
}

#[test]
fn test_piped_string() {
    assert!(piped_string(Span::new("")).is_err());
    assert!(piped_string(Span::new("| ")).is_ok());
    assert!(piped_string(Span::new("| hi")).is_ok());
}

fn tokenised_string<'a>(i: Span<'a>) -> IResult<Span<'a>, Vec<StringToken>, ParserError<Span<'a>>> {
    nom::multi::many0(alt((
        map(interpolated_variable, |v| StringToken::Variable(v)),
        map(raw_text, |s| StringToken::Text(s)),
    )))(i)
}

fn raw_text<'a>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>, ParserError<Span<'a>>> {
    is_not("\n")(i)
}

fn interpolated_variable<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Variable<'a>, ParserError<Span<'a>>> {
    nom::sequence::tuple((
        multispace0,
        tag("${"),
        multispace0,
        variable,
        multispace0,
        char('}'),
    ))(i)
    .map(|(r, (_, _, _, var, _, _))| (r, var))
    // .map_err(|e| {
    //     e.map(|(s, _k)| ParserError {
    //         context: i, // we need to reset the context to the whole line
    //         kind: ParserErrorKind::UnexpectedToken(s.fragment().to_string()),
    //         pos: s,
    //     })
    // })
}

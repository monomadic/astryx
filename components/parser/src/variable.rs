use crate::{errorold::ParserErrorKind, Literal, ParserError, Span};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::map,
    number::complete::double,
    sequence::{delimited, tuple},
    IResult,
};

pub(crate) fn literal<'a>(i: Span<'a>) -> IResult<Span<'a>, Literal<'a>, ParserError<Span<'a>>> {
    alt((
        // map(hash, JsonValue::Object),
        // map(array, JsonValue::Array),
        map(quoted_string, |s: Span| Literal::String(s)),
        // map(relative_path, |s: Span| Variable::RelativePath(s)),
        // map(alphanumeric1, |s: Span| Variable::Reference(s)),
        // map(argument_idx,   |i| Property::ArgumentIndex(i.parse::<usize>().unwrap())),
        map(double, |f| Literal::Number(i, f)),
        // map(digit1,         |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
        // map(boolean,        |b| Property::Boolean(b)),
        // map(dotted_symbol,  |s| Property::DottedSymbol(String::from(s))),
        // map(symbol,         |s| Property::Symbol(String::from(s))),
    ))(i)
    // .map_err(|e| {
    //     e.map(|e| ParserError {
    //         context: i, // we need to reset the context to the whole line
    //         kind: ParserErrorKind::UnexpectedToken("variable".into()),
    //         pos: e,
    //     })
    // })
}

// fn number<'a>(i: Span<'a>) -> IResult<Span<'a>, Literal, ParserError<Span<'a>>> {
//     let (r, f) = double(i)?;
//     Ok((r, Literal::Number(i, f)))
// }

fn quoted_string<'a>(i: Span<'a>) -> IResult<Span, Span, ParserError<Span<'a>>> {
    delimited(char('\"'), is_not("\""), char('\"'))(i)
}

/// match glob patterns eg: ./*.txt and ../../*
pub fn glob_pattern(i: Span) -> IResult<Span, Span, ParserError<Span>> {
    tuple((path_prefix, glob_pattern_characters))(i)
        .map(|(r, (_prefix, path))| (r, path)) // fix this so that prefix is included
        .map_err(|e| {
            e.map(|_| ParserError {
                kind: ParserErrorKind::Unexpected,
                pos: i,
                context: i,
            })
        })
}

/// match relative paths eg: ./test.txt and ../../test.txt
pub fn relative_path(i: Span) -> IResult<Span, Span, ParserError<Span>> {
    // we need to check for globs and return early or we'll trip up the parser with a full error condition.
    if i.contains("*") {
        return Err(nom::Err::Error(ParserError {
            context: i, // we need to reset the context to the whole line
            kind: ParserErrorKind::UnexpectedToken("gg".into()),
            pos: i,
        }));
    };

    tuple((path_prefix, path_characters))(i)
        // .map(|(r, (prefix, pathname))| (r, Span::new(&format!("{}{}", prefix, pathname)))) // check this!
        .map(|(r, (_prefix, path))| (r, path)) // fix this so that prefix is included
        .map_err(|e| {
            e.map(|_| ParserError {
                context: i, // we need to reset the context to the whole line
                kind: ParserErrorKind::UnexpectedToken("gg".into()),
                pos: i,
            })
        })
}

fn glob_pattern_characters(i: Span) -> IResult<Span, Span> {
    nom::bytes::complete::is_a("./*-_abcdefghijklmnopqrstuvwxyz1234567890ABCDEF")(i)
}

fn path_characters(i: Span) -> IResult<Span, Span> {
    nom::bytes::complete::is_a("./-_abcdefghijklmnopqrstuvwxyz1234567890ABCDEF")(i)
}

// match path prefixes ./ or ../
fn path_prefix(i: Span) -> IResult<Span, Span> {
    alt((tag("./"), tag("../")))(i)
}

impl Literal<'_> {
    pub fn inspect(&self) -> String {
        match self {
            Literal::String(s) => format!("\"{}\"", s.fragment().to_string()),
            Literal::Number(_, f) => f.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_literal() {
        assert_eq!(
            literal(Span::new_extra("4", "")).unwrap().1.inspect(),
            String::from("4")
        );
    }
}

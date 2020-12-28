use crate::Span;
use nom::{
    bytes::complete::is_not,
    character::complete::{newline, one_of, space0},
    combinator::{cut, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct Line<'a> {
    pub content: Span<'a>,
    pub children: Vec<Line<'a>>,
}

// pub(crate) fn take_lines<'a>(i: &'a str) -> IResult<Span<'a>, Vec<Line<'a>>> {
//     cut(many0(take_children))(Span::new(&i))
// }

pub(crate) fn take_lines<'a>(i: Span<'a>) -> IResult<Span<'a>, Vec<Line<'a>>> {
    cut(many0(take_children))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_take_lines() {
        assert!(take_lines(Span::new_extra("", "")).is_ok());
        assert!(take_lines(Span::new_extra("\n", "")).is_ok());

        assert_eq!(take_lines(Span::new_extra("\na\n", "")).unwrap().1.len(), 1);
        assert_eq!(take_lines(Span::new_extra("a\n", "")).unwrap().1.len(), 1);
        // assert_eq!(take_lines("a\nb").unwrap().0.to_string(), "");
        // assert_eq!(take_lines("a\nb").unwrap().1[0].content.to_string(), "a");
        // assert_eq!(take_lines("a\nb").unwrap().1[1].content.to_string(), "b");
        // assert_eq!(take_lines("a\nb").unwrap().1.len(), 2);
        // assert_eq!(
        //     take_lines("page()\nb").unwrap().1[0].content.to_string(),
        //     "page()"
        // );
        // assert_eq!(
        //     take_lines("page()\nb").unwrap().1[1].content.to_string(),
        //     "b"
        // );
        // assert!(take_lines("page\n").is_ok());

        // // test throw away blank lines
        // assert_eq!(take_lines("a\nb\n\n").unwrap().1.len(), 2);
        // assert_eq!(take_lines("a\n\nb\n\nc\n").unwrap().1.len(), 3);

        // // test children
        // assert_eq!(
        //     take_lines("a\n\tb\n").unwrap().1[0].content.to_string(),
        //     "a"
        // );
        // assert_eq!(
        //     take_lines("a\n\tb\n").unwrap().1[0].children[0]
        //         .content
        //         .to_string(),
        //     "b"
        // );
        // assert_eq!(
        //     take_lines("a\n\tb\n\tc\n").unwrap().1[0].children[1]
        //         .content
        //         .to_string(),
        //     "c"
        // );
    }
}

fn take_children(i: Span) -> IResult<Span, Line> {
    let (mut r, (indent, line)) = line(i)?;
    let mut children: Vec<Line> = Vec::new();

    // see map_while
    while line_indent(r.fragment()) > indent {
        let (rem, child) = take_children(r)?;
        children.push(child);
        r = rem;
    }

    Ok((
        r,
        Line {
            content: line,
            children,
        },
    ))
}

/// take a single line in the format (indent, content) and chomp newline
fn line(i: Span) -> IResult<Span, (usize, Span)> {
    tuple((
        opt(many0(tuple((space0, newline)))), // throw away blank lines
        nom::multi::many0_count(one_of(" \t")),
        is_not("\n"),
        opt(newline),
    ))(i)
    .map(|(r, (_, indent, line, _))| (r, (indent, line)))
}

/// returns the position of the first non-whitespace character,
/// or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

fn line_indent(i: &str) -> usize {
    indentation_level(i).map(|(_r, indent)| indent).unwrap_or(0)
}

use crate::{ Span, eof::eof};
use nom::{
    bytes::complete::{take_till, take_until, take_while, is_not},
    character::{is_alphabetic, complete::{newline, one_of, not_line_ending, line_ending, multispace0, space0, alpha1}},
    combinator::{not, cut, opt},
    multi::many0,
    sequence::{delimited, tuple, preceded, terminated},
    IResult, branch::alt,
};

/// whitespace significant linesplit
/// see: map_while

#[derive(Debug)]
pub struct Line<'a> {
    pub content: Span<'a>,
    pub children: Vec<Line<'a>>,
}

pub(crate) fn take_lines(i: &str) -> IResult<Span, Vec<Line>> {
    cut(many0(take_children))(Span::new(i))
}

#[test]
fn test_take_lines() {
    assert!(take_lines("").is_ok());
    assert!(take_lines("\n").is_ok());
    assert_eq!(take_lines("a\n").unwrap().1.len(), 1);
    assert_eq!(take_lines("a\nb").unwrap().0.to_string(), "");
    assert_eq!(take_lines("a\nb").unwrap().1[0].content.to_string(), "a");
    assert_eq!(take_lines("a\nb").unwrap().1[1].content.to_string(), "b");
    assert_eq!(take_lines("a\nb").unwrap().1.len(), 2);
    assert_eq!(take_lines("page()\nb").unwrap().1[0].content.to_string(), "page()");
    assert_eq!(take_lines("page()\nb").unwrap().1[1].content.to_string(), "b");
    assert!(take_lines("page\n").is_ok());

    // test throw away blank lines
    assert_eq!(take_lines("a\nb\n\n").unwrap().1.len(), 2);
    assert_eq!(take_lines("a\n\nb\n\nc\n").unwrap().1.len(), 3);

    // test children
    assert_eq!(take_lines("a\n\tb\n").unwrap().1[0].content.to_string(), "a");
    assert_eq!(take_lines("a\n\tb\n").unwrap().1[0].children[0].content.to_string(), "b");
    assert_eq!(take_lines("a\n\tb\n\tc\n").unwrap().1[0].children[1].content.to_string(), "c");
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
        nom::multi::many0_count(one_of(" \t")),
        is_not("\n"),
        opt(newline),
        opt(many0(tuple((space0, newline)))), // throw away blank lines
    ))
    (i)
    .map(|(r, (indent, line, _, _))| (r, (indent, line)))
}

// fn take_line(i: Span) -> IResult<Span, (usize, Span)> {
//     take_until("\n")(i).map(|(r, line)| (r, (line_indent(line.fragment()), line)))
// }

/// returns the position of the first non-whitespace character,
/// or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

fn line_indent(i: &str) -> usize {
    indentation_level(i).map(|(_r, indent)| indent).unwrap_or(0)
}

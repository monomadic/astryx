use crate::Span;
use nom::{
    bytes::complete::is_not,
    character::complete::{newline, one_of, space0},
    combinator::{cut, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};
use rctree::Node;

pub(crate) fn split(i: Span) -> IResult<Span, Vec<Node<Span>>> {
    let (r, lines) = cut(many0(line))(i)?;
    let mut current_indent = 0;

    // while let (indent, line) = lines.pop() {}

    let l = lines
        .into_iter()
        .map(|(indent, line)| {
            if indent > current_indent {}
            current_indent = indent;
            println!("indent: {:?}", indent);

            Node::new(line)
        })
        .collect();

    Ok((r, l))
}

/// take a single line in the format (indent, content) and chomp newline
fn line(i: Span) -> IResult<Span, (usize, Span)> {
    tuple((
        opt(many0(tuple((space0, newline)))), // throw away blank lines
        nom::multi::many0_count(one_of(" \t")),
        is_not("\n"),
        opt(newline),
    ))(i)
    .map(|(r, (_, indent, line, _))| (r, (indent, Span::new_extra(line.fragment(), "whoops"))))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_take_lines() {
        assert!(split(Span::new_extra("", "")).is_ok());
        assert!(split(Span::new_extra("\n", "")).is_ok());
        assert_eq!(split(Span::new_extra("a\nb", "")).unwrap().1.len(), 2);
        assert_eq!(split(Span::new_extra("a\nb\n", "")).unwrap().1.len(), 2);

        println!("====={:?}", split(Span::new_extra("a\n\tb", "")).unwrap().1);
    }
}

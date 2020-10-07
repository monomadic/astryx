use crate::Span;
use nom::{character::complete::{newline, one_of}, IResult, bytes::complete::take_until, sequence::tuple, multi::many0, combinator::cut};

/// whitespace significant linesplit
/// 

#[derive(Debug)]
pub struct Line<'a> {
    pub content: Span<'a>,
    pub children: Vec<Line<'a>>,
}

fn take_children(i: Span) -> IResult<Span, Line> {
    let (mut r, (indent, line)) = line(i)?;
    let mut children: Vec<Line> = Vec::new();

    // println!("indents: {:?}, {:?}, {}, {}", line, r, line_indent(r.fragment()), line_indent(line.fragment()));

    while line_indent(r.fragment()) > indent {
        let (rem, child) = take_children(r)?;
        children.push(child);
        r = rem;
    }

    Ok((r, Line {
        content: line,
        children,
    }))
}

pub(crate) fn take_lines(i: &str) -> IResult<Span, Vec<Line>> {
    cut(many0(take_children))(Span::new(i))
}

#[test]
fn test_take_lines() {
    // let (r, lines) = take_children(Span::new("")).unwrap();
    println!("{:#?}", take_lines("a\n\tb\nc\n\td\n"));
    // assert_eq!(take_children(Span::new(""))?.1.content, Span::new(""));
}

/// take a single line in the format (indent, content) and chomp newline
fn line(i: Span) -> IResult<Span, (usize, Span)> {
    tuple((nom::multi::many0_count(one_of(" \t")), take_until("\n"), newline))(i)
        .map(|(r, (indent, line, _))| (r, (indent, line)))
}



// fn take_child(indent: usize, line: &str) -> IResult<Span, Line> {
//     let (r, (next_indent, next_line)) = take_line(Span::new(i))?;

//     if next_indent == indent {

//     }
// }

// fn split(i: &str) -> IResult<Span, Vec<Line>> {
//     let (r, (indent, line)) = take_line(Span::new(i))?;

    


//     // match take_line(r) {
//     //     Ok((r, (next_indent, next_line))) => {
//     //         if next_indent == 
//     //     }
//     // }


//     // let mut iter = i.lines().into_iter().peekable();

//     // for line in iter {
//     //     if Some(next) = iter.peek() {
//     //         if line_indent(next) > line_indent(line) {
//     //             println!("ddd");
//     //         };
//     //     }
//     // }

//     // let current_index = line_indent(lines[0]);


//     // lines.map(|line| Line {
//     //     content: String::from(line),
//     //     children: Vec::new(),
//     // }).collect()


//     // let mut children = Vec::new();
//     // let (mut r, child) = take_line(i);

//     // while line_indent(r) > line_indent(pre) {
//     //     let (rem, child) = node(r)?;
//     //     children.push(child);
//     //     r = rem;
//     // }
// }

// #[test]
// fn test_split() {
//     assert_eq!(split("").len(), 0);
//     assert_eq!(split("a\n").len(), 1);
//     assert_eq!(split("a\n\n").len(), 2);
//     assert_eq!(split("a\n\tb\n")[0].children.len(), 1);
// }

// fn take_line_and_children(i: Span) -> IResult<Span, Line> {

// }

fn take_line(i: Span) -> IResult<Span, (usize, Span)> {
    take_until("\n")(i)
        .map(|(r, line)| (r, (line_indent(line.fragment()), line)))
}

/// returns the position of the first non-whitespace character,
/// or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

fn line_indent(i: &str) -> usize {
    indentation_level(i)
        .map(|(_r, indent)| indent)
        .unwrap_or(0)
}

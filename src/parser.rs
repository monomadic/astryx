// lexer
// tokenises an astryx program

use crate::{
    error::{AstryxError, AstryxErrorKind, AstryxResult},
    variable::Variable,
};

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::error::*;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::*;
use nom::{
    character::complete::{alphanumeric1, char, multispace0, newline, one_of, space0, space1},
    combinator::{map, opt},
};

#[derive(Debug, Clone)]
pub enum Token {
    ForLoop(ForLoop),
    Element(Element),
    Text(String),
    CodeBlock(CodeBlock),
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub ident: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub index: String,
    pub iterable: String,
    pub children: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub ident: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Token>,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    Symbol(String),
    Decorator(Decorator),
    Class(String),
    NamedAttribute { ident: String, variable: Variable },
}

#[derive(Debug, Clone)]
pub struct Decorator {
    pub ident: String,
    // value: ?
}

/// returns a vector of ast nodes
pub fn parse(i: &str) -> AstryxResult<Vec<Token>> {
    let (r, nodes) = run(i).map_err(|e| AstryxError::new(&format!("error parsing: {:?}", e)))?;

    if !r.is_empty() {
        return Err(AstryxError {
            kind: AstryxErrorKind::ParseError,
            state: None,
            msg: format!(
                "file did not fully parse.\n\nRemainder:\n{}\n\nNodes:\n{:#?}",
                r, nodes
            ),
        });
    }

    Ok(nodes)
}

/// returns a nom combinator version of the parser
pub fn run(i: &str) -> IResult<&str, Vec<Token>> {
    nom::multi::many0(node)(i)
}

// // preparse lines terminated in \ to not break
// fn strip_terminators(i: &str) -> String {
//   i.replace("\\n", "")
// }

fn node(i: &str) -> IResult<&str, Token> {
    // knock out blank lines at start of doc
    let (r, _) = blank_lines(i)?;

    alt((
        map(for_loop, |f| Token::ForLoop(f)),
        map(piped_string, |s| Token::Text(String::from(s))),
        map(codeblock, |cb| Token::CodeBlock(cb)),
        map(element, |e| Token::Element(e)),
    ))(r)
}

fn codeblock(i: &str) -> IResult<&str, CodeBlock> {
    let (mut r, (pre, ident, _, _)) = trim(nom::sequence::tuple((
        multispace0,
        symbolic1,
        char(':'),
        newline,
    )))(i)?;

    let mut children = Vec::new();

    while line_indent(r) > line_indent(pre) {
        let (rem, line) = take_until("\n")(r)?;
        children.push(line);
        r = rem;
    }

    Ok((
        r,
        CodeBlock {
            ident: ident.into(),
            content: children.join("\n"),
        },
    ))
}

fn for_loop(i: &str) -> IResult<&str, ForLoop> {
    let (mut r, (pre, _, _, ident, _, _, _, relative_path, _)) = trim(nom::sequence::tuple((
        multispace0,
        tag("for"),
        space1,
        alphanumeric1,
        space0,
        tag("in"),
        space1,
        relative_path,
        blank_lines,
    )))(i)?;

    let mut children = Vec::new();

    while line_indent(r) > line_indent(pre) {
        let (rem, child) = node(r)?;
        children.push(child);
        r = rem;
    }

    Ok((
        r,
        ForLoop {
            index: ident.into(),
            iterable: relative_path.into(),
            children,
        },
    ))
}

fn element(i: &str) -> IResult<&str, Element> {
    let (mut r, (pre, ident, _, attributes, _, _)) = nom::sequence::tuple((
        multispace0,
        symbolic1,
        space0_with_early_terminators,
        nom::multi::many0(attribute),
        newline,
        blank_lines,
    ))(i)?;

    let mut children = Vec::new();

    while line_indent(r) > line_indent(pre) {
        let (rem, child) = node(r)?;
        children.push(child);
        r = rem;
    }

    Ok((
        r,
        Element {
            ident: ident.into(),
            attributes: attributes,
            children,
        },
    ))
}

fn attribute(i: &str) -> IResult<&str, Attribute> {
    alt((
        map(decorator, |d| Attribute::Decorator(d)),
        map(dotted_symbol, |s| Attribute::Class(s)),
        attribute_assignment,
        map(symbolic1, |s| Attribute::Symbol(String::from(s))),
    ))(i)
}

fn decorator(i: &str) -> IResult<&str, Decorator> {
    let (input, (_, _, ident, _)) = nom::sequence::tuple((
        space0_with_early_terminators,
        char('@'),
        symbolic1,
        space0_with_early_terminators,
    ))(i)?;

    return Ok((
        input,
        Decorator {
            ident: String::from(ident),
        },
    ));
}

fn dotted_symbol(i: &str) -> IResult<&str, String> {
    let (input, (_, _, ident, _)) = nom::sequence::tuple((
        space0_with_early_terminators,
        char('.'),
        symbolic1,
        space0_with_early_terminators,
    ))(i)?;

    return Ok((input, String::from(ident)));
}

fn attribute_assignment(i: &str) -> IResult<&str, Attribute> {
    let (input, (_, ident, _, variable, _)) = nom::sequence::tuple((
        space0_with_early_terminators,
        symbolic1,
        char('='),
        variable,
        space0_with_early_terminators,
    ))(i)?;

    return Ok((
        input,
        Attribute::NamedAttribute {
            ident: String::from(ident),
            variable: variable,
        },
    ));
}

fn quoted_string(i: &str) -> IResult<&str, &str> {
    trim(delimited(char('\"'), is_not("\""), char('\"')))(i)
}

fn piped_string(i: &str) -> IResult<&str, &str> {
    let (r, (_, _, value, _)) =
        nom::sequence::tuple((multispace0, tag("| "), is_not("\n"), blank_lines))(i)?;

    return Ok((r, value));
}

fn relative_path(i: &str) -> IResult<&str, &str> {
    let (input, (_, path)) = nom::sequence::tuple((tag("./"), path_chars))(i)?;

    return Ok((input, path));
}

fn variable(i: &str) -> IResult<&str, Variable> {
    alt((
        // map(hash, JsonValue::Object),
        // map(array, JsonValue::Array),
        map(quoted_string, |s| Variable::QuotedString(String::from(s))),
        map(relative_path, |s| Variable::RelativePath(String::from(s))),
        map(symbolic1, |s| Variable::Reference(String::from(s))),
        // map(argument_idx,   |i| Property::ArgumentIndex(i.parse::<usize>().unwrap())),
        // map(double,         |f| Property::Float(f)),
        // map(digit1,         |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
        // map(boolean,        |b| Property::Boolean(b)),
        // map(dotted_symbol,  |s| Property::DottedSymbol(String::from(s))),
        // map(symbol,         |s| Property::Symbol(String::from(s))),
    ))(i)
}

/// returns the position of the first non-whitespace character,
/// or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

fn line_indent(i: &str) -> usize {
    let (_, indent) = indentation_level(i).unwrap_or(("", 0));
    indent
}

/// trim whitespace before a string
fn trim<'a, O1, F>(
    inner: F,
) -> impl Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>
where
    F: Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>,
{
    preceded(opt(one_of(" \t\n\r")), inner)
}

/// match valid characters for file paths
fn path_chars<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.clone().as_char();
            !(c == '-' || c == '/' || c == '.' || c == '_' || c == '*' || item.is_alphanum())
        },
        ErrorKind::AlphaNumeric,
    )
}

/// match blank lines including early terminators (\)
fn space0_with_early_terminators(i: &str) -> IResult<&str, Vec<&str>> {
    nom::multi::many0(alt((
        map(one_of(" \t"), |_| " "),
        map(tag("\\\n"), |_| " "),
    )))(i)
}

/// match blank lines including early terminators (\)
// fn space1_with_early_terminators(i: &str) -> IResult<&str, Vec<&str>> {
//     nom::multi::many1(alt((
//         map(one_of(" \t"), |_| " "),
//         map(tag("\\\n"), |_| " "),
//     )))(i)
// }

fn blank_lines(i: &str) -> IResult<&str, Vec<(&str, char)>> {
    nom::multi::many0(nom::sequence::tuple((space0, newline)))(i)
}

#[test]
fn check_blank_lines() {
    let (r, i) = blank_lines("").unwrap();
    assert_eq!(r, "");
    assert_eq!(i, vec![]);

    let (r, i) = blank_lines("  a\n").unwrap();
    assert_eq!(r, "  a\n");
    assert_eq!(i, vec![]);

    let (r, i) = blank_lines("\n").unwrap();
    assert_eq!(r, "");
    assert_eq!(i, vec![("", '\n')]);

    let (r, i) = blank_lines("\n a").unwrap();
    assert_eq!(r, " a");
    assert_eq!(i, vec![("", '\n')]);

    let (r, i) = blank_lines("\n   link").unwrap();
    assert_eq!(r, "   link");
    assert_eq!(i, vec![("", '\n')]);
}

/// valid characters for an ident
fn symbolic1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.clone().as_char();
            !(c == '-' || c == '_' || c == '.' || item.is_alphanum())
        },
        ErrorKind::AlphaNumeric,
    )
}

// tests

#[test]
fn check_element() {
    let (r, res) = element("\npage path=./index.html title=\"monomadic\"\n").unwrap();
    assert_eq!(res.ident, String::from("page"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.attributes.len(), 2);
    assert_eq!(res.children.len(), 0);
    assert_eq!(r, "");
}

#[test]
pub(crate) fn check_for_loop() {
    let (r, res) = for_loop("for x in ./local\n").unwrap();
    assert_eq!(res.index, String::from("x"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 0);
    assert_eq!(r, "");

    let (r, res) = for_loop("for post in ./posts\n\tnode\n\tanother\n").unwrap();
    assert_eq!(res.index, String::from("x"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 2);
    assert_eq!(r, "");

    let (r, res) = for_loop("for post in ./posts\n\tlink href=post\n").unwrap();
    assert_eq!(res.index, String::from("post"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 1);
    assert_eq!(r, "");
}

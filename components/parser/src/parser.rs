// Parser
// Tokenises an astryx program into an AST

use crate::{
    error::{ParserError, ParserResult},
    variable::Variable,
};
use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{alphanumeric1, char, multispace0, newline, one_of, space0, space1},
    combinator::{map, opt},
    error::*,
    sequence::{delimited, preceded, tuple},
};

#[derive(Debug, Clone)]
pub enum Token {
    Comment(String),
    ForLoop(ForLoop),
    Element(Element),
    Text(Vec<StringToken>),
    CodeBlock(CodeBlock),
}

#[derive(Debug, Clone)]
pub enum StringToken {
    Text(String),
    Variable(Variable),
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub ident: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub index: String,
    pub iterable: Variable,
    pub children: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub ident: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Token>,
    // pub text: Option<Variable>
}

impl Element {
    pub fn get_optional_attribute(&self, arg: &str) -> Option<Variable> {
        for attribute in &self.attributes {
            if let Attribute::NamedAttribute { ident, variable } = attribute {
                if ident == arg {
                    return Some(variable.clone());
                }
            }
        }
        None
    }
    pub fn get_required_attribute(&self, arg: &str) -> ParserResult<Variable> {
        self.get_optional_attribute(arg)
            .ok_or(ParserError::new(&format!("variable not found: {}", arg)))
    }
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

/// returns a nom combinator version of the parser
pub fn run(i: &str) -> IResult<&str, Vec<Token>> {
    nom::multi::many0(node)(i)
}

#[test]
fn test_run() {
    assert!(run("").is_ok());
    assert!(run("page").is_ok());
    assert!(run("page\n").is_ok());
    assert!(run("page\n\tdiv\n").is_ok());
    assert_eq!(run("page\n\n\n").unwrap().0, "");
}

fn node(i: &str) -> IResult<&str, Token> {
    // knock out blank lines at start of doc
    let (r, _) = blank_lines(i)?;

    alt((
        map(comment, |s| Token::Comment(String::from(s))),
        map(for_loop, |f| Token::ForLoop(f)),
        map(piped_string, |string_tokens| Token::Text(string_tokens)),
        map(codeblock, |cb| Token::CodeBlock(cb)),
        map(element, |e| Token::Element(e)),
    ))(r)
}

#[test]
fn test_node() {
    // test newline bounds of each node here (not nodes themselves)
    assert!(node("").is_err());
    assert_eq!(node("# comment\nelement\n").unwrap().0, "element\n");
    assert_eq!(node("for x in ./file\n\tchild\nelement\n").unwrap().0, "element\n");
}

fn comment(i: &str) -> IResult<&str, &str> {
    trim(delimited(char('#'), is_not("\n"), char('\n')))(i)
}

#[test]
fn test_comment() {
    assert!(comment("").is_err());
    assert_eq!(comment("# \n"), Ok(("", " ")));
    assert_eq!(comment("# comment\n"), Ok(("", " comment")));
    assert_eq!(
        comment("# embed path=./assets/monomadic.svg\n"),
        Ok(("", " embed path=./assets/monomadic.svg"))
    );
    assert_eq!(comment(" # comment\n"), Ok(("", " comment")));
    assert_eq!(comment("\t# comment\n"), Ok(("", " comment")));
    assert_eq!(
        comment("\t# comment\n\tanother\n"),
        Ok(("\tanother\n", " comment"))
    );
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

#[test]
fn test_codeblock() {
    assert!(codeblock("").is_err());

    let (r, cb) = codeblock("css:\n\tstyle {}\n").unwrap();
    assert_eq!(r, "\n");
    assert_eq!(cb.ident, "css");
    assert_eq!(cb.content, "\tstyle {}");
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
            iterable: Variable::RelativePath(relative_path.into()),
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
    let (input, (_, ident, _, _, _, variable, _)) = nom::sequence::tuple((
        space0_with_early_terminators,
        symbolic1,
        space0,
        char('='),
        space0,
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

fn piped_string(i: &str) -> IResult<&str, Vec<StringToken>> {
    let (r, (_, _, value, _)) =
        nom::sequence::tuple((multispace0, tag("| "), tokenised_string, newline))(i)?;

    return Ok((r, value));
}

fn tokenised_string(i: &str) -> IResult<&str, Vec<StringToken>> {
    nom::multi::many1(alt((
        map(interpolated_variable, |v| StringToken::Variable(v)),
        map(raw_text, |s| StringToken::Text(s.into())),
    )))(i)
}

fn raw_text(i: &str) -> IResult<&str, &str> {
    is_not("\n")(i)
}

fn interpolated_variable(i: &str) -> IResult<&str, Variable> {
    let (r, (_, _, _, var, _, _)) = nom::sequence::tuple((
        multispace0,
        tag("${"),
        multispace0,
        variable,
        multispace0,
        char('}'),
    ))(i)?;

    Ok((r, var))
}

fn path_characters(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::is_a("./*-_abcdefghijklmnopqrstuvwxyz1234567890ABCDEF")(i)
}

/// match relative paths eg: ./test.txt and ../../test.txt
fn relative_path(i: &str) -> IResult<&str, String> {
    let (r, (prefix, pathname)) = trim(tuple((path_prefix, path_characters)))(i)?;

    Ok((r, format!("{}{}", prefix, pathname)))
}

#[test]
fn test_relative_path() {
    assert!(relative_path("").is_err());
    assert_eq!(relative_path("./file.txt"), Ok(("", "./file.txt".into())));
    assert_eq!(
        relative_path("./file.txt\nhello"),
        Ok(("\nhello", "./file.txt".into()))
    );
}

// match path prefixes ./ or ../
fn path_prefix(i: &str) -> IResult<&str, &str> {
    alt((tag("./"), tag("../")))(i)
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

// /// match valid characters for file paths
// fn path_chars<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
// where
//     T: InputTakeAtPosition,
//     <T as InputTakeAtPosition>::Item: AsChar + Clone,
// {
//     input.split_at_position1_complete(
//         |item| {
//             let c = item.clone().as_char();
//             !(c == '-' || c == '/' || c == '.' || c == '_' || c == '*' || item.is_alphanum())
//         },
//         ErrorKind::AlphaNumeric,
//     )
// }

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
    assert_eq!(res.index, String::from("post"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 2);
    assert_eq!(r, "");

    let (r, res) = for_loop("for post in ./posts\n\tlink href=post\n").unwrap();
    assert_eq!(res.index, String::from("post"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 1);
    assert_eq!(r, "");
}

// Parser
// Tokenises an astryx program into an AST

use crate::{
    error::{ParserError, ParserResult},
    variable::Variable,
};
use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, char, multispace0, newline, one_of, space0, space1},
    combinator::{map, opt},
    error::*,
    sequence::{delimited, preceded, tuple},
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone)]
pub enum Token {
    CodeBlock(CodeBlock),
    Comment(String),
    Element(Element),
    ForLoop(ForLoop),
    FunctionCall(FunctionCall),
    Text(Vec<StringToken>),
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub ident: String,
    pub arguments: Vec<(String, Variable)>,
    pub children: Vec<Token>,
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
        unimplemented!();
        // self.get_optional_attribute(arg)
        //     .ok_or(ParserError::new(&format!("variable not found: {}", arg)))
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

// /// returns a nom combinator version of the parser
// pub fn run(i: &str) -> IResult<Span, Vec<Token>> {
//     let s = Span::new(i);
//     nom::multi::many0(node)(s)
// }

// #[test]
// fn test_run() {
//     assert!(run("").is_ok());
//     assert!(run("page").is_ok());
//     assert!(run("page\n").is_ok());
//     assert!(run("page\n\tdiv\n").is_ok());
//     assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 0);
// }

pub(crate) fn node(s: Span) -> IResult<Span, Token> {
    // // knock out blank lines at start of doc
    // let (r, _) = blank_lines(s)?;

    let (s, t) = alt((
        map(comment, |s| Token::Comment(String::from(*s.fragment()))),
        map(for_loop, |f| Token::ForLoop(f)),
        map(function_call, |f| Token::FunctionCall(f)),
        map(piped_string, |string_tokens| Token::Text(string_tokens)),
        // map(codeblock, |cb| Token::CodeBlock(cb)),
        map(element, |e| Token::Element(e)),
    ))(s)?;

    Ok((s, t))
}

// fn nnode(i: &str) -> IResult<&str, Token> {
//     // knock out blank lines at start of doc
//     let (r, _) = blank_lines(i)?;

//     alt((
//         map(comment, |s| Token::Comment(String::from(s))),
//         map(for_loop, |f| Token::ForLoop(f)),
//         map(function_call, |f| Token::FunctionCall(f)),
//         map(piped_string, |string_tokens| Token::Text(string_tokens)),
//         map(codeblock, |cb| Token::CodeBlock(cb)),
//         map(element, |e| Token::Element(e)),
//     ))(r)
// }

#[test]
fn test_node() {
    // test newline bounds of each node here (not nodes themselves)
    assert!(node(LocatedSpan::new("")).is_err());
    // assert_eq!(
    //     node(LocatedSpan::new("# comment\nelement\n")).unwrap().0,
    //     LocatedSpan::new("element\n")
    // );

    let (span, token) = node(LocatedSpan::new("for x in ./file\n\tchild\nelement\n")).unwrap();

    assert_eq!(&format!("{:?}", token), "ForLoop(ForLoop { index: \"x\", iterable: RelativePath(\"./file\"), children: [Element(Element { ident: \"child\", attributes: [], children: [] })] })");
    // assert_eq!(span.location_offset(), 0); 23
    // assert_eq!(span.location_line(), 1);
    // assert_eq!(span.get_column(), 1);
    assert_eq!(span.fragment(), &&"element\n"[..]);

    // errors

    // let Err(_e) = node(LocatedSpan::new("44"));
    // let err = node(LocatedSpan::new("44")).unwrap_err().position;
    // assert!(err.is_incomplete());
    // let (span, e) = err.unwrap();
    // assert_eq!(&format!("{:?}", err), "Error((LocatedSpan { offset: 2, line: 1, fragment: \"\", extra: () }, Char))");
}

fn comment(i: Span) -> IResult<Span, Span> {
    preceded(multispace0, delimited(char('#'), is_not("\n"), char('\n')))(i)
}

#[test]
fn test_comment() {
    assert!(comment(LocatedSpan::new("")).is_err());
    assert!(comment(LocatedSpan::new("# \n")).is_ok());
    assert!(comment(LocatedSpan::new("# comment\n")).is_ok());
    assert_eq!(
        comment(LocatedSpan::new("# embed path=./assets/monomadic.svg\n")),
        Ok((
            LocatedSpan::new(""),
            LocatedSpan::new(" embed path=./assets/monomadic.svg")
        ))
    );
    assert_eq!(
        comment(LocatedSpan::new(" # comment\n")),
        Ok((LocatedSpan::new(""), LocatedSpan::new(" comment")))
    );
    assert_eq!(
        comment(LocatedSpan::new("\t# comment\n")),
        Ok((LocatedSpan::new(""), LocatedSpan::new(" comment")))
    );
    assert_eq!(
        comment(LocatedSpan::new("\t# comment\n\tanother\n")),
        Ok((
            LocatedSpan::new("\tanother\n"),
            LocatedSpan::new(" comment")
        ))
    );
}

// fn codeblock(i: &str) -> IResult<&str, CodeBlock> {
//     let (mut r, (pre, ident, _, _)) = trim(nom::sequence::tuple((
//         multispace0,
//         symbolic1,
//         char(':'),
//         newline,
//     )))(i)?;

//     let mut children = Vec::new();

//     while line_indent(r) > line_indent(pre) {
//         let (rem, line) = take_until("\n")(r)?;
//         children.push(line);
//         r = rem;
//     }

//     Ok((
//         r,
//         CodeBlock {
//             ident: ident.into(),
//             content: children.join("\n"),
//         },
//     ))
// }

// #[test]
// fn test_codeblock() {
//     assert!(codeblock("").is_err());

//     let (r, cb) = codeblock("css:\n\tstyle {}\n").unwrap();
//     assert_eq!(r, "\n");
//     assert_eq!(cb.ident, "css");
//     assert_eq!(cb.content, "\tstyle {}");
// }

fn for_loop(i: Span) -> IResult<Span, ForLoop> {
    let (mut r, (pre, _, _, ident, _, _, _, relative_path, _)) = nom::sequence::tuple((
        multispace0,
        tag("for"),
        space1,
        alphanumeric1,
        space0,
        tag("in"),
        space1,
        relative_path,
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
        ForLoop {
            index: String::from(*ident.fragment()),
            iterable: Variable::RelativePath(relative_path.into()),
            children,
        },
    ))
}

fn function_call(i: Span) -> IResult<Span, FunctionCall> {
    let (mut r, (pre, ident, _, _, arguments, _, _, _)) = nom::sequence::tuple((
        multispace0,
        symbolic1,
        space0,
        char('('),
        nom::multi::many0(function_argument),
        char(')'),
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
        FunctionCall {
            ident: String::from(*ident.fragment()),
            arguments: arguments.into_iter().map(|(k, v)| (String::from(*k.fragment()), v)).collect(),
            children,
        },
    ))
}

// #[test]
// fn test_function_call() {
//     assert!(function_call("").is_err());

//     let (r, f) = function_call("print()\n").unwrap();
//     assert_eq!(r, "");
//     assert_eq!(f.ident, "print");
//     assert_eq!(f.arguments.len(), 0);

//     let (r, f) = function_call(r#"hello(world: "hi")\n"#).unwrap();
//     assert_eq!(r, "");
//     assert_eq!(f.ident, "page");
//     assert_eq!(f.arguments.len(), 0);

//     let (r, f) =
//         function_call(r#"page(path: "/", title: "monomadic", stylesheet: "style.css")\n"#).unwrap();
//     assert_eq!(r, "");
//     assert_eq!(f.ident, "page");
//     assert_eq!(f.arguments.len(), 0);
// }

fn function_argument(i: Span) -> IResult<Span, (Span, Variable)> {
    let (r, (_, ident, _, _, _, variable, _)) = nom::sequence::tuple((
        space0,
        symbolic1,
        space0,
        char(':'),
        space0,
        variable,
        opt(char(',')),
    ))(i)?;

    Ok((r, (ident.into(), variable)))
}

#[test]
fn test_function_argument() {
    assert!(function_argument(Span::new("")).is_err());

    let (r, (ident, variable)) = function_argument(Span::new(r#"path: "/""#)).unwrap();
    assert_eq!(*r.fragment(), "");
    assert_eq!(*ident.fragment(), "path");
    assert_eq!(variable.to_string(), "/".to_string());

    let (r, (ident, variable)) = function_argument(Span::new(r#"method: "GET","#)).unwrap();
    assert_eq!(*r.fragment(), "");
    assert_eq!(*ident.fragment(), "method");
    assert_eq!(variable.to_string(), "GET".to_string());
}

// #[test]
// pub(crate) fn test_for_loop() {
//     let (r, res) = for_loop("for x in ./local\n").unwrap();
//     assert_eq!(res.index, String::from("x"));
//     // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
//     assert_eq!(res.children.len(), 0);
//     assert_eq!(r, "");

//     let (r, res) = for_loop("for post in ./posts\n\tnode\n\tanother\n").unwrap();
//     assert_eq!(res.index, String::from("post"));
//     // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
//     assert_eq!(res.children.len(), 2);
//     assert_eq!(r, "");

//     let (r, res) = for_loop("for post in ./posts\n\tlink href=post\n").unwrap();
//     assert_eq!(res.index, String::from("post"));
//     // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
//     assert_eq!(res.children.len(), 1);
//     assert_eq!(r, "");

//     let (r, res) = for_loop(
//         "for page in ./posts/*.md\n\tdiv .post .page-link .padding-sm\n\tlink href=post\n",
//     )
//     .unwrap();
//     assert_eq!(res.index, String::from("page"));
//     assert_eq!(res.iterable.to_string(), "./posts/*.md");
//     assert_eq!(res.children.len(), 2);
//     assert_eq!(r, "");
// }

fn element(i: Span) -> IResult<Span, Element> {
    let (mut r, (pre, ident, _, attributes, _)) = nom::sequence::tuple((
        multispace0,
        symbolic1,
        space0_with_early_terminators,
        nom::multi::many0(attribute),
        newline,
        // blank_lines,
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
            ident: String::from(*ident.fragment()),
            attributes: attributes,
            children,
        },
    ))
}

fn attribute(i: Span) -> IResult<Span, Attribute> {
    alt((
        map(decorator, |d| Attribute::Decorator(d)),
        map(dotted_symbol, |s| Attribute::Class(s)),
        attribute_assignment,
        // map(symbolic1, |s| Attribute::Symbol(String::from(s))),
    ))(i)
}

fn decorator(i: Span) -> IResult<Span, Decorator> {
    let (input, (_, _, ident, _)) = nom::sequence::tuple((
        space0_with_early_terminators,
        char('@'),
        symbolic1,
        space0_with_early_terminators,
    ))(i)?;

    return Ok((
        input,
        Decorator {
            ident: String::from(*ident.fragment()),
        },
    ));
}

fn dotted_symbol(i: Span) -> IResult<Span, String> {
    let (input, (_, _, ident, _)) = nom::sequence::tuple((
        space0_with_early_terminators,
        char('.'),
        symbolic1,
        space0_with_early_terminators,
    ))(i)?;

    return Ok((input, String::from(*ident.fragment())));
}

fn attribute_assignment(i: Span) -> IResult<Span, Attribute> {
    nom::sequence::tuple((
        space0_with_early_terminators,
        symbolic1,
        space0,
        char('='),
        space0,
        variable,
        space0_with_early_terminators,
    ))(i)
    .map(|(input, (_, ident, _, _, _, variable, _))| {
        (
            input,
            Attribute::NamedAttribute {
                ident: String::from(*ident.fragment()),
                variable: variable,
            },
        )
    })
}

// todo: whitespace?
fn quoted_string(i: Span) -> IResult<Span, Span> {
    delimited(char('\"'), is_not("\""), char('\"'))(i)
}

#[test]
fn test_quoted_string() {
    assert_eq!(
        *quoted_string(Span::new("\"hi\"")).unwrap().0.fragment(),
        "hi"
    );
}

fn piped_string(i: Span) -> IResult<Span, Vec<StringToken>> {
    // let (r, (_, _, value, _)) =
    //     nom::sequence::tuple((multispace0, tag("| "), tokenised_string, newline))(i)?;

    // return Ok((r, value));

    nom::sequence::tuple((multispace0, tag("| "), tokenised_string, newline))(i)
        .map(|(r, (_, _, value, _))| (r, value))
}

#[test]
fn test_piped_string() {
    assert!(piped_string(LocatedSpan::new("| stringy\n")).is_ok());
    assert_eq!(
        *piped_string(LocatedSpan::new(
            "\t| stringy\n\n\tfor post in ./posts/*.md\n"
        ))
        .unwrap()
        .0
        .fragment(),
        "\n\tfor post in ./posts/*.md\n"
    );
}

fn tokenised_string(i: Span) -> IResult<Span, Vec<StringToken>> {
    nom::multi::many1(alt((
        map(interpolated_variable, |v| StringToken::Variable(v)),
        map(raw_text, |s| StringToken::Text(String::from(*s.fragment()))),
    )))(i)
}

fn raw_text(i: Span) -> IResult<Span, Span> {
    is_not("\n")(i)
}

fn interpolated_variable(i: Span) -> IResult<Span, Variable> {
    nom::sequence::tuple((
        multispace0,
        tag("${"),
        multispace0,
        variable,
        multispace0,
        char('}'),
    ))(i)
    .map(|(r, (_, _, _, var, _, _))| (r, var))
}

fn path_characters(i: Span) -> IResult<Span, Span> {
    nom::bytes::complete::is_a("./*-_abcdefghijklmnopqrstuvwxyz1234567890ABCDEF")(i)
}

/// match relative paths eg: ./test.txt and ../../test.txt
fn relative_path(i: Span) -> IResult<Span, String> {
    tuple((path_prefix, path_characters))(i)
        .map(|(r, (prefix, pathname))| (r, format!("{}{}", prefix, pathname)))
}

#[test]
fn test_relative_path() {
    assert!(relative_path(Span::new("")).is_err());
    assert_eq!(
        *relative_path(Span::new("./file.txt")).unwrap().0.fragment(),
        "./file.txt"
    );
    assert_eq!(
        relative_path(Span::new("./file.txt\nhello")),
        Ok((Span::new("\nhello"), String::from("./file.txt")))
    );
}

// match path prefixes ./ or ../
fn path_prefix(i: Span) -> IResult<Span, Span> {
    alt((tag("./"), tag("../")))(i)
}

fn variable(i: Span) -> IResult<Span, Variable> {
    alt((
        // map(hash, JsonValue::Object),
        // map(array, JsonValue::Array),
        map(quoted_string, |s: Span| Variable::QuotedString(String::from(*s.fragment()))),
        map(relative_path, |s: String| Variable::RelativePath(s)),
        map(symbolic1, |s: Span| Variable::Reference(String::from(*s.fragment()))),
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
fn indentation_level(i: Span) -> IResult<Span, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

fn line_indent(i: Span) -> usize {
    let (_, indent) = indentation_level(i).unwrap_or((Span::new(""), 0));
    indent
}

// fn trim<'a, O1, F>(
//     inner: F,
// ) -> impl Fn(&'a Span) -> IResult<Span, O1, (Span, nom::error::ErrorKind)>
// where
//     F: Fn(&'a Span) -> IResult<Span, O1, (Span, nom::error::ErrorKind)>,
// {
//     preceded(opt(one_of(" \t\n\r")), inner)
// }

// /// trim whitespace before a string
// fn trim(i: Span) -> IResult<Span, &str> {
//     preceded(opt(one_of(" \t\n\r")), inner)(i)
// }

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
fn space0_with_early_terminators(i: Span) -> IResult<Span, Vec<&str>> {
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

fn blank_lines(i: Span) -> IResult<Span, Vec<(Span, char)>> {
    nom::multi::many0(nom::sequence::tuple((space0, newline)))(i)
}

// #[test]
// fn check_blank_lines() {
//     let (r, i) = blank_lines("").unwrap();
//     assert_eq!(r, "");
//     assert_eq!(i, vec![]);

//     let (r, i) = blank_lines("  a\n").unwrap();
//     assert_eq!(r, "  a\n");
//     assert_eq!(i, vec![]);

//     let (r, i) = blank_lines("\n").unwrap();
//     assert_eq!(r, "");
//     assert_eq!(i, vec![("", '\n')]);

//     let (r, i) = blank_lines("\n a").unwrap();
//     assert_eq!(r, " a");
//     assert_eq!(i, vec![("", '\n')]);

//     let (r, i) = blank_lines("\n   link").unwrap();
//     assert_eq!(r, "   link");
//     assert_eq!(i, vec![("", '\n')]);
// }

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

#[test]
fn test_element() {
    let (r, res) = element(Span::new("\npage path=./index.html title=\"monomadic\"\n")).unwrap();
    assert_eq!(res.ident, String::from("page"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.attributes.len(), 2);
    assert_eq!(res.children.len(), 0);
    assert_eq!(r, Span::new(""));
}

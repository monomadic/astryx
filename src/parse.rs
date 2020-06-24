use crate::models::*;

use nom::*;
use nom::branch::alt;
use nom::combinator::{ map, opt, value };
use nom::character::complete::{ newline, space0, space1, multispace0, alphanumeric1, one_of, char, digit1 };
use nom::number::complete::{ double };
use nom::bytes::complete::{ tag, is_not, take_until };
use nom::sequence::preceded;
use nom::sequence::delimited;
use nom::error::*;

/// returns a nom combinator version of the parser
pub fn run(i: &str) -> IResult<&str, Vec<Node>> {
    nom::multi::many0(node)(i)
}

fn blank_lines(i: &str) -> IResult<&str, Vec<(&str, char)>> {
  nom::multi::many0(
    nom::sequence::tuple((space0, newline)))(i)
}

fn node(i: &str) -> IResult<&str, Node> {
  // knock out blank lines
  let (r, m) = nom::multi::many0(
    nom::sequence::tuple((space0, newline)))(i)?;

  alt((
      map(for_loop, |f| Node::ForLoop(f)),
      map(piped_string, |s| Node::Text(String::from(s))),
      map(element, |e| Node::Element(e)),
  ))(r)
}

fn for_loop(i: &str) -> IResult<&str, ForLoop> {
  let (mut r, (pre, _, _, ident, _, _, _, variable, _)) =
    trim(nom::sequence::tuple((
      multispace0,
      tag("for"),
      space1,
      symbolic1,
      space1,
      tag("in"),
      space1,
      variable,
      blank_lines
    )))(i)?;
  
  println!(">> {:?}", r);

  let mut children = Vec::new();

  println!("compare: {} {} {:?}", line_indent(pre), line_indent(r), (pre, ident, &variable));
  while line_indent(r) > line_indent(pre) {
    let (rem, child) = node(r)?;
    children.push(child);
    r = rem;
  }

  Ok((r,
    ForLoop {
      reference: ident.into(),
      iterable: variable,
      children,
    }
  ))
}

fn element(i: &str) -> IResult<&str, Element> {
	let (mut r, (pre, ident, _, attributes, _)) =
		nom::sequence::tuple((
      multispace0,
      symbolic1,
      space0,
      nom::multi::many0(attribute),
      blank_lines
    ))(i)?;

	let mut children = Vec::new();

	println!("compare: {} {} {}", line_indent(pre), line_indent(r), ident);

	while line_indent(r) > line_indent(pre) {
		let (rem, child) = node(r)?;
		children.push(child);
		r = rem;
	}

	Ok((r,
		Element {
			ident: ident.into(),
			attributes: attributes,
			children,
		}
  ))
}

fn attribute(i: &str) -> IResult<&str, Attribute> {
    alt((
        attribute_assignment,
        map(symbolic1, |s| Attribute::Symbol(String::from(s))),
    ))(i)
}

fn attribute_assignment(i: &str) -> IResult<&str, Attribute> {
	let (input, (_, ident, _, variable, _)) =
		nom::sequence::tuple(
			(multispace0, symbolic1, char('='), variable, space0)
		)(i)?;

	return Ok((input,
		Attribute::Assignment {
			ident: String::from(ident),
			variable: variable,
		}
  ))
}

fn quoted_string(i: &str) -> IResult<&str, &str> {
    trim(delimited(
        char('\"'), is_not("\""), char('\"')
    ))(i)
}

fn piped_string(i: &str) -> IResult<&str, &str> {
	let (r, (_, _, value, _)) =
		nom::sequence::tuple((
      multispace0, tag("| "), is_not("\n"), blank_lines
    ))(i)?;

	return Ok((r, value))
}

fn relative_path(i: &str) -> IResult<&str, &str> {
	let (input, (_, path)) =
		nom::sequence::tuple((
      tag("./"),
      path_chars
    ))(i)?;
	
	return Ok((input,
		path
	))
}

fn variable(i: &str) -> IResult<&str, Variable> {
    alt((
        // map(hash, JsonValue::Object),
        // map(array, JsonValue::Array),
        map(quoted_string,  |s| Variable::QuotedString(String::from(s))),
        map(relative_path,  |s| Variable::RelativePath(String::from(s))),
        map(symbolic1,      |s| Variable::Reference(String::from(s))),
        // map(argument_idx,   |i| Property::ArgumentIndex(i.parse::<usize>().unwrap())),
        // map(double,         |f| Property::Float(f)),
        // map(digit1,         |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
        // map(boolean,        |b| Property::Boolean(b)),
        // map(dotted_symbol,  |s| Property::DottedSymbol(String::from(s))),
        // map(symbol,         |s| Property::Symbol(String::from(s))),
    ))(i)
}

/// returns the position of the first non-whitespace character, or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
  // let (r, line) = take_until("\n")(i)?;
  nom::multi::many0_count(one_of(" \t"))(i)
}

fn line_indent(i: &str) -> usize {
	let (_, indent) = indentation_level(i).unwrap_or(("",0));
	indent
}

/// trim whitespace before a string
fn trim<'a, O1, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>
where F: Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>,
{
    preceded(opt(one_of(" \t\n\r")), inner)
}

/// valid characters for a file path
fn path_chars<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input.split_at_position1_complete(|item| {
    let c = item.clone().as_char();
    !(c == '-' || c == '/' || c == '.' || c == '_' || item.is_alphanum())
  },
    ErrorKind::AlphaNumeric
  )
}

/// valid characters for an ident
fn symbolic1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input.split_at_position1_complete(|item| {
    let c = item.clone().as_char();
    !(c == '-' || c == '_' || c == '.' || item.is_alphanum())
  },
    ErrorKind::AlphaNumeric
  )
}

// take until newline occurs (FIXME: include \)
fn take_while_newline(i: &str) -> IResult<&str, &str> {
  nom::bytes::complete::take_while(|c| c == '\n')(i)
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
fn check_for_loop() {
    let (r, res) = for_loop("for x in ./local\n").unwrap();
    assert_eq!(res.reference, String::from("x"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 0);
    assert_eq!(r, "");

    let (r, res) = for_loop("for x in ./local.txt\n\tnode\n\tanother\n").unwrap();
    assert_eq!(res.reference, String::from("x"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 2);
    assert_eq!(r, "");

    let (r, res) = for_loop("for post in ./posts\n\tlink href=post\n").unwrap();
    assert_eq!(res.reference, String::from("post"));
    // assert_eq!(res.iterable, Variable::RelativePath(String::from("local")));
    assert_eq!(res.children.len(), 1);
    assert_eq!(r, "");
}

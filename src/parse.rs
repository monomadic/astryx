use crate::models::*;

use nom::*;
use nom::branch::alt;
use nom::combinator::{ map, opt, value };
use nom::character::complete::{ space0, space1, multispace0, alphanumeric1, one_of, char, digit1 };
use nom::number::complete::{ double };
use nom::bytes::complete::{ tag, is_not };
use nom::sequence::preceded;
use nom::sequence::delimited;
use nom::error::*;

/// returns a nom combinator version of the parser
pub fn run(i:&str) -> IResult<&str, Vec<Node>> {
    nom::multi::many0(node)(i)
}

fn node(i: &str) -> IResult<&str, Node> {
    alt((
        map(element, |e| Node::Element(e)),
        map(quoted_string, |s| Node::Text(String::from(s))),
    ))(i)
}

fn element(i: &str) -> IResult<&str, Element> {
	let (input, (_, ident, _, attributes, _)) =
		nom::sequence::tuple(
			(multispace0, symbolic1, space0, nom::multi::many0(attribute), take_while_newline)
		)(i)?;

	return Ok((input,
		Element {
			ident: ident.into(),
			attributes: attributes,
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

fn relative_path(i: &str) -> IResult<&str, &str> {
	let (input, (_, _, path, _)) =
		nom::sequence::tuple(
			(char('.'), char('/'), path_chars, space0)
		)(i)?;
	
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
        // map(argument_idx,   |i| Property::ArgumentIndex(i.parse::<usize>().unwrap())),
        // map(double,         |f| Property::Float(f)),
        // map(digit1,         |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
        // map(boolean,        |b| Property::Boolean(b)),
        // map(dotted_symbol,  |s| Property::DottedSymbol(String::from(s))),
        // map(symbol,         |s| Property::Symbol(String::from(s))),
    ))(i)
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
    !(c == '-' || c == '_' || item.is_alphanum())
  },
    ErrorKind::AlphaNumeric
  )
}

// take until newline occurs (FIXME: include \)
fn take_while_newline(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(|c| c == '\n')(i)
}

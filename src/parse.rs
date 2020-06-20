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
		(multispace0, symbolic1, space0, nom::multi::many0(symbolic1), take_while_newline)
	)(i)?;

	return Ok((input,
		Element {
			ident: ident.into(),
		}
    ))
}

fn quoted_string(i: &str) -> IResult<&str, &str> {
    trim(delimited(
        char('\"'), is_not("\""), char('\"')
    ))(i)
}

/// trim whitespace before a string
fn trim<'a, O1, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>
where F: Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>,
{

    preceded(opt(one_of(" \t\n\r")), inner)
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

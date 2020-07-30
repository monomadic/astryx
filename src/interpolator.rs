// takes input from the parser and constructs a node graph
// TODO convert to a tree adt

use crate::{error::*, variable::{stringify_variable, Variable}};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::map;
use nom::*;
use nom::{self, character::complete::multispace0};
use nom::{
    character::complete::char,
    error::{ErrorKind, ParseError},
};
use std::collections::HashMap;

pub(crate) fn interpolate(i: &str, locals: &HashMap<String, Variable>) -> AstryxResult<String> {
    let (r, nodes) = run(i).expect("interpolation failed");
    let mut output_buffer = String::new();

    for node in nodes {
        match node {
            InterpolationNode::Text(t) => {
                output_buffer.push_str(&t);
            }
            InterpolationNode::Reference(r) => {
                output_buffer.push_str(&stringify_variable(&Variable::Reference(r), locals)?);
            }
        }
    }

    output_buffer.push_str(r); // push any remainder as well (text at the end of the line)

    Ok(output_buffer)
}

#[test]
fn check_interpolate() {
    let r = interpolate("this is a ${ post }.", &HashMap::new()).unwrap();
    assert_eq!(r, "f");
}

fn run(i: &str) -> IResult<&str, Vec<InterpolationNode>> {
    nom::multi::many0(interpolation_node)(i)
}

#[derive(Debug)]
enum InterpolationNode {
    Text(String),
    Reference(String),
}

fn interpolation_node(i: &str) -> IResult<&str, InterpolationNode> {
    alt((
        map(interpolate_reference, |r| {
            InterpolationNode::Reference(r.into())
        }),
        map(interpolate_text, |r| InterpolationNode::Text(r.into())),
    ))(i)
}

fn interpolate_reference(i: &str) -> IResult<&str, &str> {
    let (r, (_, _, _, reference, _, _)) = nom::sequence::tuple((
        multispace0,
        tag("${"),
        multispace0,
        symbolic1,
        multispace0,
        char('}'),
    ))(i)?;

    Ok((r, reference))
}

#[test]
fn check_interpolate_reference() {
    let (r, i) = interpolate_reference("${ post.url }").unwrap();
    assert_eq!(r, "");
    assert_eq!(i, "post.url");
}

fn interpolate_text(i: &str) -> IResult<&str, &str> {
    take_until("${")(i)
    // is_not("$")(i)
}

#[test]
fn check_interpolate_text() {
    let (r, i) = interpolate_text("blah ${ post.url }").unwrap();
    assert_eq!(r, "${ post.url }");
    assert_eq!(i, "blah ");
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

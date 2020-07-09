use crate::{error::*, models::*};
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
use yaml_rust::Yaml;

pub fn interpolate(i: &str, locals: &HashMap<String, Variable>) -> ParseResult<String> {
    let (r, nodes) = run(i).expect("interpolation failed");
    let mut output_buffer = String::new();

    for node in nodes {
        match node {
            InterpolationNode::Text(t) => {
                output_buffer.push_str(&t);
            }
            InterpolationNode::Reference(r) => {
                // FIXME unsafe
                let base_ref: &str = r.split(".").collect::<Vec<&str>>()[0];

                output_buffer.push_str(&stringify_variable(
                    &get_required_variable(&base_ref, &locals)?,
                    locals,
                )?);
            }
        }
    }

    output_buffer.push_str(r);

    Ok(output_buffer)
}

#[test]
pub(crate) fn check_interpolate() {
    let r = interpolate("this is a ${ post }.", &HashMap::new()).unwrap();
    assert_eq!(r, "f");
}

// interpolation

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
pub(crate) fn check_interpolate_reference() {
    let (r, i) = interpolate_reference("${ post.url }").unwrap();
    assert_eq!(r, "");
    assert_eq!(i, "post.url");
}

fn interpolate_text(i: &str) -> IResult<&str, &str> {
    take_until("${")(i)
    // is_not("$")(i)
}

#[test]
pub(crate) fn check_interpolate_text() {
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

// fn strings1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
// where
//     T: InputTakeAtPosition,
//     <T as InputTakeAtPosition>::Item: AsChar + Clone,
// {
//     input.split_at_position1_complete(
//         |item| {
//             let c = item.clone().as_char();
//             !(c != '-' || c == '_' || c == '.' || item.is_alphanum())
//         },
//         ErrorKind::AlphaNumeric,
//     )
// }

// FIXME (changed) duplicated code from interpeter.rs - impl display?
pub fn stringify_variable(
    variable: &Variable,
    locals: &HashMap<String, Variable>,
) -> ParseResult<String> {
    match variable {
        Variable::RelativePath(p) => Ok(p.clone()),
        Variable::Reference(p) => {
            // FIXME unsafe array index
            let base_ref: &str = p.split(".").collect::<Vec<&str>>()[0];
            let subref: &str = p.split(".").collect::<Vec<&str>>()[1];

            if let Some(Variable::TemplateFile(template_file)) =
                locals.get(base_ref)
            {
                let yaml_var = template_file.metadata.clone().unwrap()[subref].clone();

                match yaml_var {
                    Yaml::String(s) => Ok(s),
                    _ => Err(AstryxError::ParseError(format!(
                        "reference_not_found: {}",
                        subref
                    ))),
                }
            } else {
                locals
                    .get(p)
                    .ok_or(AstryxError::ParseError(format!(
                        "reference_not_found: {} {:?}",
                        &p, &locals
                    )))
                    .and_then(|v| stringify_variable(v, locals))
            }
        }
        Variable::QuotedString(p) => Ok(p.clone()),
        Variable::TemplateFile(t) => {
            println!("== {:?}", t);
            Ok(t.body.clone())
        },
    }
}

// FIXME dupe code from interpreter.rs
pub fn get_required_variable(
    i: &str,
    attributes: &HashMap<String, Variable>,
) -> ParseResult<Variable> {
    attributes
        .get(&String::from(i.clone()))
        .map(|v| v.clone().clone())
        .ok_or(AstryxError::ParseError(format!(
            "could not find variable: {} {:?}",
            i,
            attributes.keys()
        )))
}

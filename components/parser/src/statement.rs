use crate::{
    element::element,
    errorold::ParserErrorKind,
    function::function_call,
    models::Statement,
    text::piped_string,
    variable::{glob_pattern, literal, relative_path},
    Expression, ParserError, Route, Span,
};
use error::{AstryxError, AstryxErrorKind, Location};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, space0, space1},
    combinator::{all_consuming, cut, map},
    multi::many0,
    sequence::{terminated, tuple},
    Err, IResult,
};
use nom_locate::LocatedSpan;
use rctree::Node;

pub(crate) fn span_to_location(span: Span) -> Location {
    Location {
        line: span.location_line(),
        column: span.get_column(),
        length: span.location_offset(),
        filename: span.extra.into(),
        context: String::from_utf8(span.get_line_beginning().into()).unwrap(),
    }
}

// fn array<'a>(i: Span) -> IResult<Span, Span, ParserError<Span>> {
//     // fn letter(i: &str) -> IResult<&str, Token, ParserError> {
//     tuple((char('['), alpha1, char(']')))(i)
//         .map(|(r, (_, ident, _))| (r, ident))
//         .map_err(|e| {
//             e.map(|(span, _kind)| ParserError {
//                 context: span,
//                 kind: ParserErrorKind::SyntaxError,
//                 pos: span.into(),
//             })
//         })
// }

// #[test]
// fn test_array() {
//     assert!(array(Span::new("[g]")).is_ok());
// }

/// Attempt to parse a Statement tree from a Span tree.
pub(crate) fn statement_node<'a>(
    node: Node<Span<'a>>,
) -> IResult<Span, Node<Statement<'a>>, AstryxError> {
    let (rem, stmt) = statement(node.borrow().clone())?;
    let mut stmt_node = Node::new(stmt);

    for child in node.children() {
        stmt_node.append({
            let (rem, st) = statement_node(child)?;
            // return an error if there is unconsumed input
            if !rem.is_empty() {
                return Err(Err::Error(AstryxError::LocatedError(
                    span_to_location(rem),
                    AstryxErrorKind::UnexpectedToken(rem.to_string()),
                )));
            }
            st
        });
    }

    Ok((rem, stmt_node))
}

/// Attempt to parse a Statement from a text Span
pub fn statement<'a, S: Into<LocatedSpan<&'a str, &'a str>>>(
    i: S,
) -> IResult<Span<'a>, Statement<'a>, AstryxError> {
    all_consuming(alt((
        // map(function_call, |f| Statement::FunctionCall(f)),
        map(comment, Statement::Comment),
        map(for_loop, |(ident, expr)| Statement::ForLoop { ident, expr }),
        map(binding, |(ident, expr)| Statement::Binding(ident, expr)),
        map(expression, Statement::Expression),
        map(route, Statement::Route),
        map(element, Statement::Element),
        map(piped_string, Statement::Text),
        map(space0, |ws| Statement::Blank(ws)),
        // map(alpha1, |e| Statement::Element(e)),
        // return_statement
    )))(i.into())
    .map_err(|e| {
        e.map(|e| {
            AstryxError::LocatedError(
                span_to_location(e.context),
                AstryxErrorKind::Unimplemented(format!("{:?}", e)), // fixme: account for actual parser errors
            )
        })
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn expect_statement(tests: Vec<(&str, &str)>) {
        for (input, expected) in tests {
            match statement(Span::from(input)) {
                Ok(obj) => {
                    assert!(obj.0.is_empty());
                    assert_eq!(expected, obj.1.to_string(), "for `{}`", input);
                }
                Err(err) => {
                    panic!(
                        "expected `{}`, but got error=`{}` for `{}`",
                        expected, err, input
                    );
                }
            }
        }
    }

    #[test]
    fn test_statement() {
        expect_statement(vec![
            ("", ""),
            (" ", ""),
            ("5", "Expression(Literal(Number(5)))"),
            ("a", "Expression(Reference(a))"),
            ("h1 {}", "Expression(FunctionCall(Reference(h1)))"),
            ("print()", "Expression(FunctionCall(Reference(print)))"),
        ]);
    }
}

fn for_loop<'a>(i: Span<'a>) -> IResult<Span, (Span<'a>, Expression<'a>), ParserError<Span<'a>>> {
    tuple((
        tag("for"),
        space1,
        alphanumeric1,
        terminated(space1, tag("in")),
        space1,
        expression,
    ))(i)
    .map(|(r, (_, _, ident, _, _, expr))| (r, (ident, expr)))
}

#[derive(Debug, Clone)]
pub struct ForLoop<'a> {
    pub index: Span<'a>,
    pub iterable: Expression<'a>,
}

fn route<'a>(i: Span<'a>) -> IResult<Span<'a>, Route<'a>, ParserError<Span<'a>>> {
    tuple((tag("@"), alphanumeric1, space0, many0(attribute_assignment)))(i)
        .map(|(r, (_, ident, _, attributes))| (r, Route { ident, attributes }))
        .map_err(|e: nom::Err<_>| {
            e.map(|e: ParserError<Span<'a>>| ParserError {
                context: e.context,
                kind: ParserErrorKind::SyntaxError,
                pos: i.into(),
            })
        })
}

// todo: replace
fn attribute_assignment(i: Span) -> IResult<Span, (Span, Expression), ParserError<Span>> {
    nom::sequence::tuple((
        multispace0,
        alpha1,
        terminated(multispace0, char('=')),
        space0,
        cut(expression),
    ))(i)
    .map(|(r, (_, ident, _, _, value))| (r, (ident, value)))
}

pub(crate) fn expression<'a>(i: Span<'a>) -> IResult<Span, Expression<'a>, ParserError<Span<'a>>> {
    alt((
        map(index, |(index, expr)| {
            Expression::Index(Box::new(index), Box::new(expr))
        }),
        map(relative_path, |s| Expression::RelativePath(s)),
        map(glob_pattern, |s| Expression::GlobPattern(s)),
        map(function_call, |f| Expression::FunctionCall(f)),
        map(literal, |v| Expression::Literal(v)),
        map(alphanumeric1, |s| Expression::Reference(s)),
    ))(i)
}

fn index(i: Span) -> IResult<Span, (Expression, Expression), ParserError<Span>> {
    tuple((index_expression, tag("."), expression))(i)
        .map(|(r, (index, _, expr))| (r, (index, expr)))
    // separated_list(tag("."), expression)(i)
    // tag("--")(i).map(|(r, _)| (Span::new(""), r))
}

fn index_expression<'a>(i: Span<'a>) -> IResult<Span, Expression<'a>, ParserError<Span<'a>>> {
    alt((
        //map(relative_path, |s| Expression::RelativePath(s)),
        map(glob_pattern, |s| Expression::GlobPattern(s)),
        map(function_call, |f| Expression::FunctionCall(f)),
        map(literal, |v| Expression::Literal(v)),
        map(alphanumeric1, |s| Expression::Reference(s)),
    ))(i)
    .map_err(|_err| {
        nom::Err::Error(ParserError {
            kind: ParserErrorKind::Unexpected,
            pos: i,
            context: i,
        })
    })
}

fn comment(i: Span) -> IResult<Span, Span, ParserError<Span>> {
    tag("--")(i).map(|(r, _)| (Span::new_extra("", ""), r)) // FXIME
                                                            // .map_err(|e| {
                                                            //     e.map(|(span, _kind)| ParserError {
                                                            //         context: span,
                                                            //         kind: ParserErrorKind::SyntaxError,
                                                            //         pos: span.into(),
                                                            //     })
                                                            // })
}

fn binding<'a>(i: Span<'a>) -> IResult<Span, (Span<'a>, Expression<'a>), ParserError<Span<'a>>> {
    tuple((
        tag("let"),
        space1,
        alphanumeric1,
        terminated(space0, tag("=")),
        space0,
        expression,
    ))(i)
    .map(|(r, (_, _, ident, _, _, expr))| (r, (ident, expr)))
}

use crate::{
    element::element,
    function::function_call,
    models::Statement,
    text::piped_string,
    variable::{glob_pattern, literal, relative_path},
    Expression, ParserError, Span,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, space1},
    combinator::{all_consuming, map},
    sequence::{terminated, tuple},
    IResult,
};

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

pub(crate) fn statement<'a>(i: Span<'a>) -> IResult<Span, Statement<'a>, ParserError<Span<'a>>> {
    all_consuming(alt((
        // map(function_call, |f| Statement::FunctionCall(f)),
        map(comment, |s| Statement::Comment(s)),
        map(for_loop, |(ident, expr)| Statement::ForLoop { ident, expr }),
        map(binding, |(ident, expr)| Statement::Binding(ident, expr)),
        map(expression, |e| Statement::Expression(e)),
        map(element, |e| Statement::Element(e)),
        map(piped_string, |e| Statement::Text(e)),
        // map(alpha1, |e| Statement::Element(e)),
        // return_statement
    )))(i)
    .map_err(|e| {
        e.map(|s| ParserError {
            context: i, // we need to reset the context to the whole line
            kind: s.kind,
            pos: s.pos,
        })
    })
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

pub(crate) fn expression<'a>(i: Span<'a>) -> IResult<Span, Expression<'a>, ParserError<Span<'a>>> {
    alt((
        map(function_call, |f| Expression::FunctionCall(f)),
        //map(relative_path, |s| Expression::RelativePath(s)),
        map(glob_pattern, |s| Expression::GlobPattern(s)),
        map(literal, |v| Expression::Literal(v)),
        map(alphanumeric1, |s| Expression::Reference(s)),
    ))(i)
}

fn comment<'a>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>, ParserError<Span<'a>>> {
    tag("--")(i).map(|(r, _)| (Span::new(""), r))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expression() {
        assert!(expression(Span::new("./posts/*.md")).is_ok());
    }

    #[test]
    fn test_for_loop() {
        // println!("{:?}", for_loop(Span::new("for x in ./posts/*.md")));
        assert!(for_loop(Span::new("for x in ./posts/*.md")).is_ok());
    }

    #[test]
    fn test_binding() {
        assert!(binding(Span::new("let a=5")).is_ok());
        // assert_eq!(binding(Span::new("let a=5")).unwrap().0.fragment().to_string(), "a");
        assert!(binding(Span::new("let a = 5")).is_ok());
        assert!(binding(Span::new("let print = print()")).is_ok());
        assert!(binding(Span::new("let print = fn print()")).is_ok());
        assert!(binding(Span::new("g()")).is_err());
    }

    #[test]
    fn test_statement() {
        assert!(statement(Span::new("")).is_err()); // do not allow blank lines to slip through
        assert!(statement(Span::new("g()")).is_ok());
        assert!(statement(Span::new("for x in ./posts/*.md")).is_ok());
    }
}

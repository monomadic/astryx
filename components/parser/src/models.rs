use crate::Span;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    Binding(Span<'a>, Expression<'a>),
    Route(Route<'a>),           // todo: remove
    Element(Element<'a>),       // todo: remove?
    Text(Vec<StringToken<'a>>), // todo: replace with interpolatedstring
    Comment(Span<'a>),
    ForLoop {
        ident: Span<'a>,
        expr: Expression<'a>,
    },
    Blank(Span<'a>),
}

impl Statement<'_> {
    pub fn inspect(&self) -> String {
        match self {
            Statement::Expression(e) => e.inspect(),
            Statement::Binding(_, _) => unimplemented!(),
            Statement::Element(_) => unimplemented!(),
            Statement::Text(_) => unimplemented!(),
            Statement::Comment(_) => unimplemented!(),
            Statement::ForLoop { ident: _, expr: _ } => unimplemented!(),
            Statement::Route(_) => unimplemented!(),
            Statement::Blank(_) => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    FunctionCall(FunctionCall<'a>),
    GlobPattern(Span<'a>),
    RelativePath(Span<'a>),
    Reference(Span<'a>),
    Literal(Literal<'a>),
    Array(Vec<Expression<'a>>),
    Index(Box<Expression<'a>>, Box<Expression<'a>>), // eg a.b(), "hi".log(), a.b.c
}

impl Expression<'_> {
    pub fn inspect(&self) -> String {
        match self {
            Expression::FunctionCall(f) => f.inspect(),
            Expression::GlobPattern(p) => p.to_string(),
            Expression::RelativePath(p) => p.to_string(),
            Expression::Reference(span) => span.fragment().to_string(),
            Expression::Literal(l) => l.inspect(),
            Expression::Array(_) => unimplemented!(),
            Expression::Index(i, e) => format!("{}.{}", i.inspect(), e.inspect()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall<'a> {
    pub ident: Box<Expression<'a>>,
    pub arguments: FunctionCallArguments<'a>,
}

// currently unused
#[derive(Debug, Clone)]
pub enum FunctionCallArguments<'a> {
    None,
    Named(Vec<(Span<'a>, Expression<'a>)>),
    Unnamed(Vec<Expression<'a>>),
}

impl Display for FunctionCallArguments<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCallArguments::None => f.write_str("None"),
            FunctionCallArguments::Named(_args) => f.write_str("Named"),
            FunctionCallArguments::Unnamed(_args) => f.write_str("Unnamed"),
        }
    }
}

impl FunctionCall<'_> {
    pub fn inspect(&self) -> String {
        // fixme: do not use debug, properly display args
        format!("{}({:?})", self.ident.inspect(), self.arguments)
    }
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    String(Span<'a>),
    Number(Span<'a>, f64),
}

// impl Literal {
//     pub fn to_object(&self) -> Object {
//         match self {
//             Literal::String(s) => Object::String(s),
//             Literal::Float(_, _) => {}
//         }
//     }
// }

// impl Literal<'a> {
//     pub fn to_string(&self) -> String {
//         match self {
//             Literal::String(s) => s.fragment().to_string(),
//             Literal::Float(_s, f) => f.to_string(),
//         }
//     }
// }

impl<'a> Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// impl <'a>Into<String> for Literal<'a> {
//     fn into(self) -> String {
//         match self {
//             Literal::String(s) => s.fragment().to_string(),
//             Literal::Float(_s, f) => f.to_string(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Element<'a> {
    pub ident: Span<'a>,
    pub attributes: Vec<(Span<'a>, Expression<'a>)>,
    pub text: Option<Vec<StringToken<'a>>>,
}

#[derive(Debug, Clone)]
pub struct Route<'a> {
    pub ident: Span<'a>,
    pub attributes: Vec<(Span<'a>, Expression<'a>)>,
}

#[derive(Debug, Clone)]
pub enum StringToken<'a> {
    Text(Span<'a>),
    Expression(Expression<'a>),
}

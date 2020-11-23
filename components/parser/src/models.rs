use crate::Span;
use std::fmt::Display;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token {
//     Blank,
//     Letter,
// }

// #[derive(Debug, Clone)]
// pub enum Expression<'a> {
// }

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    Binding(Span<'a>, Expression<'a>),
    Element(Element<'a>),
    Text(Vec<StringToken<'a>>), // todo: replace with interpolatedstring
    Comment(Span<'a>),
    ForLoop {
        ident: Span<'a>,
        expr: Expression<'a>,
    },
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
    Index(Box<Expression<'a>>, Box<Expression<'a>>),
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
    pub arguments: Vec<(Span<'a>, Expression<'a>)>,
}

impl FunctionCall<'_> {
    pub fn inspect(&self) -> String {
        format!(
            "{}({})",
            self.ident.inspect(),
            self.arguments
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v.inspect()))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

// #[derive(Debug, Clone)]
// pub enum Variable<'a> {
//     RelativePath(Span<'a>),
//     QuotedString(Span<'a>), // todo: make this Value(Value)?
//     Reference(Span<'a>),
//     // TemplateFile(TemplateFile),
//     // FunctionCall()
// }

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
}

#[derive(Debug, Clone)]
pub enum StringToken<'a> {
    Text(Span<'a>),
    Expression(Expression<'a>),
}

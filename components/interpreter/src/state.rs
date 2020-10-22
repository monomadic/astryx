use crate::{models::Value, InterpreterError, InterpreterResult};
use parser::{Expression, StringToken};
use std::{collections::HashMap, fs::OpenOptions, io::Write};

type LocalData<'a> = HashMap<String, Expression<'a>>;

#[derive(Debug, Clone)]
pub enum Writer {
    None,
    StdOut,
    File(String),
}

#[derive(Debug, Clone)]
pub struct State<'a> {
    locals: LocalData<'a>,
    // pub(crate) pages: Layouts,
    // pub(crate) imports: Imports,
    // pub(crate) pwd: String,
    // document: Node<AstryxNode>, // cursor ref to the current node in the output tree
    pub writer: Writer,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        State {
            locals: LocalData::new(),
            // document: Node::new(AstryxNode::Root),
            writer: Writer::None,
        }
    }

    /// bind a variable to local state
    pub fn bind(&mut self, ident: &str, expr: Expression<'a>) -> InterpreterResult<()> {
        let _ = self.locals.insert(ident.into(), expr); // return doesn't matter as all state is mutable
        Ok(()) // force return ok (this could change if mutability rules change)
    }

    pub fn get_mut_writer(&mut self) -> InterpreterResult<Box<dyn Write>> {
        match &self.writer {
            Writer::None => Err(InterpreterError::NoWriter),
            Writer::StdOut => Ok(Box::new(std::io::stdout())),
            Writer::File(path) => Ok(Box::new(
                OpenOptions::new().append(true).open(path).expect("45"),
            )),
        }
    }

    // pub fn set_writer(&mut self, writer: Writer)

    // pub fn push_element(&mut self, el: Element) -> InterpreterResult<()> {
    //     let node = Node::new(AstryxNode::HTMLElement(HTMLElement::new("hi", HashMap::new()).unwrap()));

    //     let nodeptr = node.downgrade();

    //     self.document.append(node);

    //     // let parent = &mut self.document.downgrade();
    //     // parent.append(node);

    //     self.document = nodeptr.upgrade().expect("node to upgrade");

    //     // self.document.append(node);

    //     // for child in self.document.children() {
    //     //     println!("child {:?}", child);
    //     // }

    //     Ok(())
    // }

    pub fn eval(&self, expr: &Expression) -> InterpreterResult<Value> {
        Ok(match expr {
            Expression::FunctionCall(f) => Value::String(format!("{:?}", f)),
            Expression::Reference(r) => Value::String(format!("r{:?}", r)),
            Expression::Literal(l) => Value::String(l.to_string()),
        })
    }

    pub fn render(&self) {
        // html::render::render(self.document.root());
    }

    /// Convert string tokens to a fully interpolated string
    pub fn interpolate(&self, components: Vec<StringToken>) -> InterpreterResult<String> {
        Ok(components
            .into_iter()
            .map(|st| match st {
                StringToken::Text(span) => Ok(span.fragment().to_string()),
                StringToken::Expression(expr) => self.eval(&expr).map(|e| e.into()),
            })
            .collect::<Result<Vec<String>, InterpreterError>>()?
            .into_iter()
            .collect())
    }

    pub fn write(&mut self, i: &str) -> InterpreterResult<()> {
        let mut writer = self.get_mut_writer()?;

        writer
            .write_fmt(format_args!("{}", i))
            .map_err(|_| InterpreterError::Generic("IO".into()))
    }
}

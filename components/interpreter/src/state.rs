use crate::{models::Value, AstryxNode, InterpreterError, InterpreterResult};
use html::HTMLElement;
use parser::{Element, Expression, StringToken};
use rctree::Node;
use std::collections::HashMap;

type LocalData = HashMap<String, Value>;

#[derive(Debug)]
pub struct State {
    locals: LocalData,
    // pub(crate) pages: Layouts,
    // pub(crate) imports: Imports,
    // pub(crate) pwd: String,
    document: Node<AstryxNode>, // cursor ref to the current node in the output tree
}

impl State {
    pub fn new() -> Self {
        State {
            locals: LocalData::new(),
            document: Node::new(AstryxNode::Root),
        }
    }

    pub fn bind(&mut self, ident: &str, value: Value) -> InterpreterResult<()> {
        match self.locals.insert(ident.into(), value) {
            Some(_) => Ok(()),
            None => Err(InterpreterError::Unhandled),
        }
    }

    pub fn push_element(&mut self, el: Element) -> InterpreterResult<()> {
        let node = Node::new(AstryxNode::HTMLElement(HTMLElement::new("hi", HashMap::new()).unwrap()));

        let nodeptr = node.downgrade();

        self.document.append(node);

        // let parent = &mut self.document.downgrade();
        // parent.append(node);

        self.document = nodeptr.upgrade().expect("node to upgrade");

        // self.document.append(node);

        // for child in self.document.children() {
        //     println!("child {:?}", child);
        // }

        Ok(())
    }

    pub fn eval(&self, expr: &Expression) -> InterpreterResult<Value> {
        Ok(match expr {
            Expression::FunctionCall(f) => Value::String(format!("{:?}", f)),
            Expression::Reference(r) => Value::String(format!("{:?}", r)),
            Expression::Literal(v) => Value::String(format!("{:?}", v)),
        })
    }

    pub fn render(&self) {
        // html::render::render(self.document.root());
    }

    pub fn interpolate(&self, components: Vec<StringToken>) -> InterpreterResult<String> {
        Ok(components
            .into_iter()
            .map(|st| match st {
                StringToken::Text(span) => span.fragment().to_string(),
                StringToken::Variable(v) => String::new(),
            })
            .collect::<Vec<String>>()
            .into_iter()
            .collect())
    }
}

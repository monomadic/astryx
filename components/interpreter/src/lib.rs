//! This crate takes an ast and emits a tree of interpreted nodes ready for rendering.
//! It is not responsible for output.
//!
//! It is responsible for:
//! - reading and verifying referenced files
//! - resolving variables and references into values
//! - executing functions
//!

use parser::Statement;
use rctree::Node;

mod eval;
use error::AstryxResult;
use models::{object::Object, state::State};
use std::cell::RefCell;
use std::rc::Rc;
pub mod builtins;
mod util;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run<'a>(
    nodes: &Vec<Node<Statement<'a>>>,
    state: Rc<RefCell<State>>,
) -> AstryxResult<Vec<Node<Object>>> {
    let inner = &builtins::import(state);

    nodes
        .iter()
        .map(|node| eval::eval_statement(node, Rc::clone(inner)))
        .collect::<AstryxResult<Vec<Node<Object>>>>()
}

/// evaluate a single expression with a given state
pub fn eval<'a>(statement: Statement<'a>, state: Rc<RefCell<State>>) -> AstryxResult<Node<Object>> {
    eval::eval_statement(&Node::new(statement), Rc::clone(&state))
}

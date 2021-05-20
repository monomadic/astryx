//! This crate takes an ast and emits a tree of interpreted nodes ready for rendering.
//! It is not responsible for output.
//!
//! It is responsible for:
//! - reading and verifying referenced files
//! - resolving variables and references into values
//! - executing functions
//!

use error::AstryxResult;
use models::{object::Object, state::State};
use parser::Statement;
use rctree::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub mod builtins;
mod expression;
mod statement;
mod util;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run(
    nodes: &Vec<Node<Statement>>,
    state: Rc<RefCell<State>>,
) -> AstryxResult<Vec<Node<Object>>> {
    let inner = &builtins::import(state);

    nodes
        .iter()
        .map(|node| statement::eval_statement(Rc::clone(inner), node))
        .collect::<AstryxResult<Vec<Node<Object>>>>()
}

/// evaluate a single expression with a given state
pub fn eval(statement: Statement, state: Rc<RefCell<State>>) -> AstryxResult<Node<Object>> {
    statement::eval_statement(Rc::clone(&state), &Node::new(statement))
}

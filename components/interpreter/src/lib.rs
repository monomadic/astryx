//! This crate takes an ast and emits a tree of interpreted nodes ready for rendering.
//! It is not responsible for output.
//!
//! It is responsible for:
//! - reading and verifying referenced files
//! - resolving variables and references into values
//! - executing functions
//!

// pub use error::{InterpreterError, InterpreterErrorKind};
// pub use models::State;

use parser::Statement;
use rctree::Node;

// mod error;
mod eval;
// mod state;
use error::AstryxResult;
// pub use models::Object;
use models::{object::Object, state::State};
use std::cell::RefCell;
use std::rc::Rc;
pub mod builtins;
mod util;

// pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run<'a>(nodes: &Vec<Node<Statement<'a>>>, state: Rc<RefCell<State>>) -> AstryxResult<()> {
    let inner = &builtins::import(state);

    for node in nodes {
        eval::eval_statement(&node, Rc::clone(inner))?;
    }

    // let program = inner.borrow().get_program();

    Ok(())
}

/// evaluate a single expression with a given state
pub fn eval<'a>(statement: Statement<'a>, state: Rc<RefCell<State>>) -> AstryxResult<Node<Object>> {
    eval::eval_statement(&Node::new(statement), Rc::clone(&state))
}

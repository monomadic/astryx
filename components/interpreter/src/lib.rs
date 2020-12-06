//! This crate takes an ast and emits a tree of interpreted nodes ready for rendering.
//! It is not responsible for output.
//!
//! It is responsible for:
//! - reading and verifying referenced files
//! - resolving variables and references into values
//! - executing functions
//!

pub use error::{InterpreterError, InterpreterErrorKind};
pub use state::State;

use parser::Statement;
use rctree::Node;

mod error;
mod eval;
mod models;
mod state;
pub use models::Object;
use program::ProgramInstruction;
use std::cell::RefCell;
use std::rc::Rc;
pub mod builtins;
mod util;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run<'a>(
    nodes: &Vec<Node<Statement<'a>>>,
    state: Rc<RefCell<State>>,
) -> InterpreterResult<Vec<ProgramInstruction>> {
    let inner = &builtins::import(state);

    for node in nodes {
        eval::eval_statement(&node, Rc::clone(inner))?;
    }

    let program = inner.borrow().get_program();

    Ok(program)
}

/// evaluate a single expression with a given state
pub fn eval<'a>(statement: Statement<'a>, state: Rc<RefCell<State>>) -> InterpreterResult<Object> {
    eval::eval_statement(&Node::new(statement), Rc::clone(&state))
}

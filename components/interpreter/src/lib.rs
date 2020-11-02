//! This crate takes an ast and emits a tree of interpreted nodes ready for rendering.
//! It is not responsible for output.
//!
//! It is responsible for:
//! - reading and verifying referenced files
//! - resolving variables and references into values
//! - executing functions
//!

pub use error::InterpreterError;
pub use models::AstryxNode;
pub use state::{State, Writer};

use parser::Statement;
use rctree::Node;

mod error;
mod eval;
mod models;
// mod run;
mod state;
use std::cell::RefCell;
use std::rc::Rc;
mod builtins;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run<'a>(
    nodes: &Vec<Node<Statement<'a>>>,
    state: Rc<RefCell<State<'a>>>,
) -> InterpreterResult<()> {
    // println!("run {:?}", nodes);

    // let state = Rc::new(RefCell::new(state));
    // let builtins = Rc::new(RefCell::new(builtins::builtin_state()));

    // let inner = state.borrow_mut();
    let inner = builtins::import(state);

    // let inner_state = Rc::new(RefCell::new(State::extend(state)));

    nodes
        .iter()
        .map(|node| eval::eval_statement(node, Rc::clone(&inner)))
        .collect()
}

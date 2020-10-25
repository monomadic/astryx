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
mod models;
mod run;
mod state;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run<'a>(nodes: Vec<Node<Statement<'a>>>, state: &'a mut State<'a>) -> InterpreterResult<()> {
    // println!("run {:?}", nodes);
    nodes
        .iter()
        .map(|node| run::eval_statement(node, state))
        .collect()
}

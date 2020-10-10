//! This crate takes an ast and emits a tree of interpreted nodes ready for rendering.
//! It is not responsible for output.
//! 
//! It is responsible for:
//! - reading and verifying referenced files
//! - resolving variables and references into values
//! - executing functions
//!

use parser::Statement;
pub use error::InterpreterError;
use models::AstryxNode;
use state::State;

mod run;
mod error;
mod models;
mod state;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run(statements: &Vec<Statement>) -> InterpreterResult<AstryxNode> {
    run::interpret(
        &statements[0],
        State::new(),
    )
}

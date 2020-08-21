//! This crate parses astryx source into an AST (abstract syntax tree).
//!
//! There are a few stages to this:
//! 1. Lexical analysis: breaks up the raw text into tokens
//! 2. Parsing: transforms tokens into the AST
//!
//! ## Usage
//! ```
//! use parser;
//!
//! let source = "page";
//! let ast = parser::parse(source).unwrap();
//!
//! ```

use error::{ParserError, ParserResult};
pub use parser::{run, Token};
pub mod error;
mod parser;
pub mod variable;

/// returns a vector of ast nodes
pub fn parse(i: &str) -> ParserResult<Vec<Token>> {
    let (r, nodes) = run(i).map_err(|e| ParserError::new(&format!("error parsing: {:?}", e)))?;

    if !r.is_empty() {
        return Err(ParserError::new(&format!(
            "file did not fully parse.\n\nRemainder:\n{}\n\nNodes:\n{:#?}",
            r, nodes
        )));
    };

    Ok(nodes)
}

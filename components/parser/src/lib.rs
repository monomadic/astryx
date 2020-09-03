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
//! let source = "page\n";
//! let ast = parser::parse(source).unwrap();
//!
//! ```

pub mod error;
pub mod parser;
pub mod variable;
pub use crate::parser::{run, Token};
pub use crate::error::{ParserError, ParserResult};

/// returns a vector of ast nodes
/// 
/// ``` rust
/// use parser;
/// 
/// let source = "page\n";
/// let ast = parser::parse(source);
/// 
/// assert_eq!(ast.is_ok(), true);
/// ```
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

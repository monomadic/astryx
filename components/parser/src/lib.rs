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

pub mod error;

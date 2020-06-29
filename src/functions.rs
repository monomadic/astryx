use crate::error::*;
#[derive(Debug)]
pub struct Function {
    pub ident: String,
    pub description: String,
    // pub extends: String, // ident of another function (this should be included in the instantiation)
    pub attributes: Vec<FunctionAttribute>,
    // pub execute: fn() -> ParseResult<()>,
}

#[derive(Debug)]
pub struct FunctionAttribute {
    ident: String,
    // type:  // only strings for now
    required: bool,
}

// impl Function {
//     /// extend a function with an inherited function (passed function takes precedence)
//     pub fn extend(&self, function: Function) -> ParseResult<()> {
//         Ok(())
//     }
// }

pub fn load_stdlib() -> Vec<Function> {
    vec![
        // Function {}
    ]
}

// fn fn_page() -> ParseResult<()> {
//     Ok(())
// }

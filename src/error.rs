use std::error::Error;

pub type ParseResult<T> = Result<T, Box<dyn Error>>;

// #[derive(Debug)]
// pub enum ParseError {
//     FunctionNotFound(String),
//     LocalNotFound(String),
//     UnknownBlock(String),
//     UnknownStyle(String, String),
//     ParameterMissing(String, String),
// }

// impl error::Error for CassetteError {
//     // fn description(&self) -> &str {
//     //     &format!("{}", self)
//     // }
// }

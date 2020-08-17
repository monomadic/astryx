
#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: Location,
}
pub enum ParseErrorType {
    
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

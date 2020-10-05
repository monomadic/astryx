pub type HTMLResult<T> = Result<T, HTMLError>;

#[derive(Debug, PartialEq)]
pub enum HTMLError {
    InvalidHTMLTag,
}

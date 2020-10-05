pub type InterpreterResult<T> = Result<T, InterpreterError>;

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    OrphanTag,
    EmptyFileGlob,
    InvalidGlobPattern,
    InvalidDocuments,
    UnexpectedFunction,
    MissingRequiredArgument,
    UnresolvedReferece,
    MetadataError,

    // html
    InvalidHTMLTag,
}

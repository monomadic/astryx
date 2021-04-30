use quick_error::quick_error;

pub mod display;

pub type AstryxResult<T> = Result<T, AstryxError>;

quick_error! {
    #[derive(Debug)]
    pub enum AstryxError {
        /// An error with an associated file context
        LocatedError(location: Location, kind: AstryxErrorKind) {
        } // todo: should contain a pathbuf for source file

        /// A generic error without context
        Generic(err: String) {
            display("generic error: {}", err)
        }

        /// File or IO error
        IO(err: std::io::Error) {
            from()
        }
    }
}

// #[derive(Debug)]
// pub enum AstryxError {
//     /// An error with an associated file context
//     LocatedError(Location, AstryxErrorKind), // todo: should contain a pathbuf for source file
//
//     /// A generic error without context
//     Generic(String),
//
//     /// File or IO error
//     IO(std::io::Error),
// }

// impl AstryxError {
//     pub fn with_loc<L: Into<Location>>(loc: L, kind: AstryxErrorKind) -> Self {
//         AstryxError::LocatedError(loc.into(), kind)
//     }
// }

// impl std::error::Error for AstryxError {}

// impl std::fmt::Display for AstryxError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match *self {
//             // DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
//             // // The wrapped error contains additional information and is available
//             // // via the source() method.
//             // DoubleError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
//             AstryxError::LocatedError(_, _) => write!(f, "error at:"),
//             AstryxError::IO(_) => write!(f, "io error"),
//             AstryxError::Generic(ref msg) => write!(f, "generic error: {}", msg),
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub enum AstryxErrorKind {
    SyntaxError,
    FunctionArgumentError,
    MissingRequiredArgument(String),
    Unexpected,
    UnexpectedToken(String),
    ExpectedValue,
    UnknownValue(String),
    Unimplemented(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub line: u32,
    pub column: usize,
    pub length: usize,
    pub filename: String,
    pub context: String,
}

// impl<'a> From<Span<'a>> for Location {
//     fn from(span: Span) -> Self {
//         Self {
//             line: span.location_line(),
//             column: span.get_column(),
//             length: span.location_offset(),
//             filename: span.extra.into(),
//             context: String::from_utf8(span.get_line_beginning().into()).unwrap(),
//         }
//     }
// }
//
// impl<'a> From<ParserError<Span<'a>>> for AstryxError {
//     fn from(e: ParserError<Span<'a>>) -> AstryxError {
//         AstryxError::LocatedError(e.pos.into(), AstryxErrorKind::Unexpected) // FIXME
//     }
// }

// impl<'a> From<std::io::Error> for AstryxError {
//     fn from(e: std::io::Error) -> AstryxError {
//         AstryxError::IO(e)
//     }
// }

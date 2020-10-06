// NOTE:
// this is only included because nom5, for over a year, somehow neglected to
// add it?!

use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult, InputLength,
};

pub fn eof<I: InputLength + Copy, E: ParseError<I>>(input: I) -> IResult<I, I, E> {
    if input.input_len() == 0 {
        Ok((input, input))
    } else {
        Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
    }
}

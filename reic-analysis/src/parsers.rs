use nom::{error::ParseError, IResult};

pub mod coff;
pub mod mz;
pub mod pe32;

pub trait BinParsable {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E>
    where
        Self: Sized;
}

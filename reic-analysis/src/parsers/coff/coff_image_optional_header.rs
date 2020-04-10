use crate::parsers::BinParsable;

use nameof::name_of;
use nom::combinator::{map, map_opt};
use nom::error::context;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use nom::{error::ParseError, number::complete::le_u16, IResult};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u16)]
pub enum COFFImageOptionalHeaderType {
    /// 0x107: ROM image
    ROM = 0x107,
    /// 0x10B: PE32 executable
    PE32 = 0x10B,
    /// 0x20B: PE32+ executable (64-bit)
    PE32Plus = 0x20B,
}

impl BinParsable for COFFImageOptionalHeaderType {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type COFFImageOptionalHeaderType),
            map_opt(le_u16, Self::from_u16),
        )(i)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct COFFImageStandardOptionalHeader {
    /// The unsigned integer that identifies the state of the image file.
    /// The most common number is 0x10B, which identifies it as a normal executable file.
    /// 0x107 identifies it as a ROM image, and 0x20B identifies it as a PE32+ executable.
    pub magic: COFFImageOptionalHeaderType,
    /// The linker major version number.
    pub major_linker_version: u8,
    /// The linker minor version number.
    pub minor_linker_version: u8,
    /// The size of the code (text) section,
    /// or the sum of all code sections if there are multiple sections.
    pub size_of_code: u32,
    /// The size of the initialized data section,
    /// or the sum of all such sections if there are multiple data sections.
    pub size_of_initialized_data: u32,
    /// The size of the uninitialized data section (BSS),
    /// or the sum of all such sections if there are multiple BSS sections.
    pub size_of_uninitialized_data: u32,
    /// The address of the entry point relative to the image base when
    /// the executable file is loaded into memory.
    /// For program images, this is the starting address.
    /// For device drivers, this is the address of the initialization function.
    /// An entry point is optional for DLLs.
    /// When no entry point is present, this field must be zero.
    pub address_of_entry_point: u32,
    /// The address that is relative to the image base of the
    /// beginning-of-code section when it is loaded into memory.
    pub base_of_code: u32,
}

impl BinParsable for COFFImageStandardOptionalHeader {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type COFFImageStandardOptionalHeader),
            map(
                tuple((
                    COFFImageOptionalHeaderType::try_parse,
                    le_u8,
                    le_u8,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                )),
                |p| Self {
                    magic: p.0,
                    major_linker_version: p.1,
                    minor_linker_version: p.2,
                    size_of_code: p.3,
                    size_of_initialized_data: p.4,
                    size_of_uninitialized_data: p.5,
                    address_of_entry_point: p.6,
                    base_of_code: p.7,
                },
            ),
        )(i)
    }
}

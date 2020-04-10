mod headers;
pub use headers::*;

mod section;
pub use section::*;

use crate::parsers::coff::{
    COFFHeader, COFFImageOptionalHeaderType, COFFImageStandardOptionalHeader,
};
use crate::parsers::mz::MZHeader;
use crate::parsers::BinParsable;

use nameof::name_of;
use nom::combinator::map_opt;
use nom::{
    bytes::complete::tag, bytes::complete::take, combinator::verify, error::context,
    error::ParseError, multi::count, sequence::tuple, IResult,
};
use num_traits::FromPrimitive;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct PE32Image {
    pub mz_header: MZHeader,
    pub coff_header: COFFHeader,
    pub coff_optional_header: COFFImageStandardOptionalHeader,
    pub pe32_optional_header: PE32OptionalHeader,
    pub data_directories: HashMap<DataDirectoryType, DataDirectory>,
    pub sections: Vec<Section>,
}

impl BinParsable for PE32Image {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(image: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(name_of!(type PE32Image), |file: &'a [u8]| {
            let (_, mz_header) = context(
                "PE header offset sanity check",
                verify(MZHeader::try_parse, |mz| file.len() > mz.e_lfanew as usize),
            )(file)?;

            let (i, (_, _, coff_header, coff_optional_header, pe32_optional_header)) =
                tuple((
                    take(mz_header.e_lfanew),
                    tag(b"PE\0\0"),
                    context(
                        "Check size of PE32 optional COFF header",
                        verify(COFFHeader::try_parse, |coff| {
                            // COFF standard fields + PE32 base_of_data + windows/loader-specific fields
                            coff.size_of_optional_header >= (24 + 4 + 64)
                        }),
                    ),
                    context(
                        "Check if optional header specifies PE32",
                        verify(COFFImageStandardOptionalHeader::try_parse, |coff| {
                            coff.magic == COFFImageOptionalHeaderType::PE32
                        }),
                    ),
                    context(
                        "Verify file and section alignment values",
                        verify(PE32OptionalHeader::try_parse, |header| {
                            let PE32OptionalHeaderWindowsSpecific {
                                file_alignment,
                                section_alignment,
                                ..
                            } = header.windows_specific;
                            file_alignment >= 512
                                && file_alignment <= 65536
                                && file_alignment.is_power_of_two()
                                && section_alignment >= file_alignment
                        }),
                    ),
                ))(file)?;

            let (i, data_dirs) = context(
                "Parse data directories",
                count(
                    DataDirectory::try_parse,
                    pe32_optional_header
                        .windows_specific
                        .number_of_rva_and_sizes as _,
                ),
            )(i)?;

            let data_directories = data_dirs
                .into_iter()
                .enumerate()
                .filter(|(_, dir_entry)| dir_entry.size != 0)
                .map(|(index, value)| {
                    let key = KnownDataDirectoryType::from_usize(index)
                        .map_or(DataDirectoryType::Unknown(index), |t| {
                            DataDirectoryType::Known(t)
                        });
                    (key, value)
                })
                .collect();

            let (i, sections) = context(
                "Sections",
                context(
                    "Verify sections",
                    verify(
                        count(
                            context(
                                "Get section data",
                                map_opt(
                                    context(
                                        "Validate header",
                                        verify(SectionHeader::try_parse, |header| {
                                            header.verify(
                                                pe32_optional_header
                                                    .windows_specific
                                                    .file_alignment,
                                                pe32_optional_header
                                                    .windows_specific
                                                    .section_alignment,
                                            )
                                        }),
                                    ),
                                    |header| Section::from_file_and_header(image, header),
                                ),
                            ),
                            coff_header.number_of_sections as usize,
                        ),
                        |sections| Section::verify_section_order(&sections),
                    ),
                ),
            )(i)?;

            Ok((
                i,
                Self {
                    mz_header,
                    coff_header,
                    coff_optional_header,
                    pe32_optional_header,
                    data_directories,
                    sections,
                },
            ))
        })(image)
    }
}

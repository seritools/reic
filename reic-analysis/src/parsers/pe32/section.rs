use crate::parsers::BinParsable;

use bitflags::bitflags;
use nameof::name_of;
use nom::{
    bytes::complete::take,
    combinator::map,
    combinator::map_res,
    error::context,
    error::ParseError,
    number::complete::{le_u16, le_u32},
    sequence::tuple,
    IResult,
};
use std::ffi::CString;
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct Section {
    pub header: SectionHeader,
    pub data: Vec<u8>,
    pub uninitialized_data_size: u32,
}

impl Section {
    pub fn from_file_and_header(file: &[u8], header: SectionHeader) -> Option<Self> {
        file.get(
            header.pointer_to_raw_data as usize
                ..(header.pointer_to_raw_data as usize) + (header.size_of_raw_data as usize),
        )
        .map(|data| Self {
            uninitialized_data_size: if header.size_of_raw_data < header.virtual_size {
                header.virtual_size - header.size_of_raw_data
            } else {
                0
            },
            data: data.to_vec(),
            header,
        })
    }

    pub fn verify_section_order(sections: &[Section]) -> bool {
        let mut earliest_section_start = 0;
        for section in sections {
            if section.header.virtual_address < earliest_section_start {
                return false;
            }

            earliest_section_start = section.header.virtual_address + section.header.virtual_size
        }

        true
    }
}

impl fmt::Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of!(type Section))
            .field(name_of!(header in Section), &self.header)
            .field(
                name_of!(data in Section),
                &(
                    format!("Vec<u8>, len: {:X}", self.data.len()),
                    if self.data.len() > 16 {
                        &self.data[..16]
                    } else {
                        &self.data[..]
                    },
                ),
            )
            .field(
                name_of!(uninitialized_data_size in Section),
                &self.uninitialized_data_size,
            )
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SectionHeader {
    /// An 8-byte, null-padded UTF-8 encoded string. If the string is exactly 8 characters long,
    /// there is no terminating null.
    ///
    /// For longer names, this field contains a slash (/) that is
    /// followed by an ASCII representation of a decimal number that is an offset into the string
    /// table. Executable images do not use a string table and do not support section names longer
    /// than 8 characters. Long names in object files are truncated if they are emitted to an
    /// executable file.
    pub name: String,

    // question: physical_address is in a union with virtual_size in the win sdk definition, why?
    //     union {
    //            DWORD   PhysicalAddress;
    //            DWORD   VirtualSize;
    //    } Misc;
    /// The total size of the section when loaded into memory.
    ///
    /// If this value is greater than SizeOfRawData, the section is zero-padded.
    /// This field is valid only for executable images and should be set to zero for object files.
    pub virtual_size: u32,

    /// For executable images, the address of the first byte of the section relative to the image
    /// base when the section is loaded into memory. For object files, this field is the address
    /// of the first byte before relocation is applied; for simplicity, compilers should set
    /// this to zero.
    ///
    /// Otherwise, it is an arbitrary value that is subtracted from offsets during relocation.
    pub virtual_address: u32,

    /// The size of the section (for object files) or the size of the initialized data
    /// on disk (for image files).
    ///
    /// For executable images, this must be a multiple of FileAlignment from the optional header.
    /// If this is less than VirtualSize, the remainder of the section is zero-filled.
    /// Because the SizeOfRawData field is rounded but the VirtualSize field is not, it is possible
    /// for SizeOfRawData to be greater than VirtualSize as well. When a section contains only
    /// uninitialized data, this field should be zero.
    pub size_of_raw_data: u32,

    /// The file pointer to the first page of the section within the COFF file.
    ///
    /// For executable images, this must be a multiple of FileAlignment from the optional header.
    /// For object files, the value should be aligned on a 4-byte boundary for best performance.
    /// When a section contains only uninitialized data, this field should be zero.
    pub pointer_to_raw_data: u32,

    /// The file pointer to the beginning of relocation entries for the section.
    ///
    /// This is set to zero for executable images or if there are no relocations.
    pub pointer_to_relocations: u32,

    /// The file pointer to the beginning of line-number entries for the section.
    ///
    /// This is set to zero if there are no COFF line numbers.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_linenumbers: u32,

    /// The number of relocation entries for the section. This is set to zero for executable images.
    pub number_of_relocations: u16,

    /// The number of line-number entries for the section.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_linenumbers: u16,

    /// The flags that describe the characteristics of the section.
    pub characteristics: SectionCharacteristics,
}

impl SectionHeader {
    pub fn verify(&self, file_alignment: u32, section_alignment: u32) -> bool {
        self.virtual_address % section_alignment == 0
            && self.size_of_raw_data % file_alignment == 0
            && self.pointer_to_raw_data % file_alignment == 0
    }
}

impl BinParsable for SectionHeader {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type SectionHeader),
            map(
                tuple((
                    map_res(take(8usize), |raw_name_bytes| {
                        let parsed = CString::new(raw_name_bytes).unwrap_or_else(|e| {
                            let nul_position = e.nul_position();
                            let content_up_to_nul = {
                                let mut vec = e.into_vec();
                                vec.truncate(nul_position);
                                vec
                            };
                            CString::new(content_up_to_nul).unwrap()
                        });

                        parsed.into_string()
                    }), // name
                    le_u32,                            // virtual_size
                    le_u32,                            // virtual_address
                    le_u32,                            // size_of_raw_data
                    le_u32,                            // pointer_to_raw_data
                    le_u32,                            // pointer_to_relocations
                    le_u32,                            // pointer_to_linenumbers
                    le_u16,                            // number_of_relocations
                    le_u16,                            // number_of_linenumbers
                    SectionCharacteristics::try_parse, // characteristics
                )),
                |p| Self {
                    name: p.0,
                    virtual_size: p.1,
                    virtual_address: p.2,
                    size_of_raw_data: p.3,
                    pointer_to_raw_data: p.4,
                    pointer_to_relocations: p.5,
                    pointer_to_linenumbers: p.6,
                    number_of_relocations: p.7,
                    number_of_linenumbers: p.8,
                    characteristics: p.9,
                },
            ),
        )(i)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct SectionCharacteristics: u32 {
        // 0x00000000
        // Reserved for future use.
        // 0x00000001
        // Reserved for future use.
        // 0x00000002
        // Reserved for future use.
        // 0x00000004
        // Reserved for future use.
        // 0x00000010
        // Reserved for future use.
        // 0x00000400
        // Reserved for future use.

        /// The section should not be padded to the next boundary.
        ///
        /// This flag is obsolete and is replaced by IMAGE_SCN_ALIGN_1BYTES.
        /// This is valid only for object files.
        const TYPE_NO_PAD = 0x0000_0008;
        /// The section contains executable code.
        const CNT_CODE = 0x0000_0020;
        /// The section contains initialized data.
        const CNT_INITIALIZED_DATA = 0x0000_0040;
        /// The section contains uninitialized data.
        const CNT_UNINITIALIZED_DATA = 0x0000_0080;
        /// Reserved for future use.
        const LNK_OTHER = 0x0000_0100;
        /// The section contains comments or other information. The .drectve section has this type.
        /// This is valid for object files only.
        const LNK_INFO = 0x0000_0200;
        /// The section will not become part of the image. This is valid only for object files.
        const LNK_REMOVE = 0x0000_0800;
        /// The section contains COMDAT data.
        /// For more information, see COMDAT Sections (Object Only).
        /// This is valid only for object files.
        const LNK_COMDAT = 0x0000_1000;
        /// The section contains data referenced through the global pointer (GP).
        const GPREL = 0x0000_8000;
        /// Reserved for future use.
        const MEM_PURGEABLE = 0x0002_0000;
        /// Reserved for future use.
        const MEM_16BIT = 0x0002_0000;
        /// Reserved for future use.
        const MEM_LOCKED = 0x0004_0000;
        /// Reserved for future use.
        const MEM_PRELOAD = 0x0008_0000;
        /// Align data on a 1-byte boundary. Valid only for object files.
        const ALIGN_1BYTES = 0x0010_0000;
        /// Align data on a 2-byte boundary. Valid only for object files.
        const ALIGN_2BYTES = 0x0020_0000;
        /// Align data on a 4-byte boundary. Valid only for object files.
        const ALIGN_4BYTES = 0x0030_0000;
        /// Align data on an 8-byte boundary. Valid only for object files.
        const ALIGN_8BYTES = 0x0040_0000;
        /// Align data on a 16-byte boundary. Valid only for object files.
        const ALIGN_16BYTES = 0x0050_0000;
        /// Align data on a 32-byte boundary. Valid only for object files.
        const ALIGN_32BYTES = 0x0060_0000;
        /// Align data on a 64-byte boundary. Valid only for object files.
        const ALIGN_64BYTES = 0x0070_0000;
        /// Align data on a 128-byte boundary. Valid only for object files.
        const ALIGN_128BYTES = 0x0080_0000;
        /// Align data on a 256-byte boundary. Valid only for object files.
        const ALIGN_256BYTES = 0x0090_0000;
        /// Align data on a 512-byte boundary. Valid only for object files.
        const ALIGN_512BYTES = 0x00A0_0000;
        /// Align data on a 1024-byte boundary. Valid only for object files.
        const ALIGN_1024BYTES = 0x00B0_0000;
        /// Align data on a 2048-byte boundary. Valid only for object files.
        const ALIGN_2048BYTES = 0x00C0_0000;
        /// Align data on a 4096-byte boundary. Valid only for object files.
        const ALIGN_4096BYTES = 0x00D0_0000;
        /// Align data on an 8192-byte boundary. Valid only for object files.
        const ALIGN_8192BYTES = 0x00E0_0000;
        /// The section contains extended relocations.
        const LNK_NRELOC_OVFL = 0x0100_0000;
        /// The section can be discarded as needed.
        const MEM_DISCARDABLE = 0x0200_0000;
        /// The section cannot be cached.
        const MEM_NOT_CACHED = 0x0400_0000;
        /// The section is not pageable.
        const MEM_NOT_PAGED = 0x0800_0000;
        /// The section can be shared in memory.
        const MEM_SHARED = 0x1000_0000;
        /// The section can be executed as code.
        const MEM_EXECUTE = 0x2000_0000;
        /// The section can be read.
        const MEM_READ = 0x4000_0000;
        /// The section can be written to.
        const MEM_WRITE = 0x8000_0000;
    }
}

impl BinParsable for SectionCharacteristics {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type SectionCharacteristics),
            map(le_u32, Self::from_bits_truncate),
        )(i)
    }
}

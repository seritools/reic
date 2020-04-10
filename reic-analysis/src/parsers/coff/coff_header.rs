use crate::parsers::BinParsable;

use nameof::name_of;

use bitflags::bitflags;
use nom::error::context;
use nom::sequence::tuple;
use nom::{
    combinator::{map, map_opt},
    error::ParseError,
    number::complete::{le_u16, le_u32},
    IResult,
};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, Eq)]
pub struct COFFHeader {
    /// The number that identifies the type of target machine.
    pub machine_type: MachineType,
    /// The number of sections. This indicates the size of the section table,
    /// which immediately follows the headers.
    ///
    /// Note that the Windows loader limits the number of sections to 96.
    pub number_of_sections: u16,
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value),
    /// that indicates when the file was created.
    pub time_date_stamp: u32,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_symbol_table: u32,
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_symbols: u32,
    /// The size of the optional header, which is required for executable files but not for object files.
    /// his value should be zero for an object file.
    pub size_of_optional_header: u16,
    /// The flags that indicate the attributes of the file.
    pub characteristics: Characteristics,
}

impl BinParsable for COFFHeader {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type COFFHeader),
            map(
                tuple((
                    MachineType::try_parse,
                    le_u16,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u16,
                    Characteristics::try_parse,
                )),
                |p| Self {
                    machine_type: p.0,
                    number_of_sections: p.1,
                    time_date_stamp: p.2,
                    pointer_to_symbol_table: p.3,
                    number_of_symbols: p.4,
                    size_of_optional_header: p.5,
                    characteristics: p.6,
                },
            ),
        )(i)
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum MachineType {
    /// The contents of this field are assumed to be applicable to any machine type
    Unknown = 0x0,
    /// Useful for indicating we want to interact with the host and not a WoW guest.
    TargetHost = 0x0001,
    /// x64
    AMD64 = 0x8664,
    /// Intel 386 or later processors and compatible processors
    I386 = 0x14c,

    /// RISC-V 32-bit address space
    RISCV32 = 0x5032,
    /// RISC-V 64-bit address space
    RISCV64 = 0x5064,
    /// RISC-V 128-bit address space
    RISCV128 = 0x5128,
    /// Matsushita AM33
    AM33 = 0x1d3,
    /// ARM little endian
    ARM = 0x1c0,
    /// ARM64 little endian
    ARM64 = 0xaa64,
    /// ARM Thumb-2 little endian
    ARMNT = 0x1c4,
    /// EFI byte code
    EBC = 0xebc,
    /// Intel Itanium processor family
    IA64 = 0x200,
    /// Mitsubishi M32R little endian
    M32R = 0x9041,
    /// MIPS16
    MIPS16 = 0x266,
    /// MIPS with FPU
    MIPSFPU = 0x366,
    /// MIPS16 with FPU
    MIPSFPU16 = 0x466,
    /// Power PC little endian
    POWERPC = 0x1f0,
    /// Power PC with floating point support
    POWERPCFP = 0x1f1,
    /// MIPS big-endian
    R3000BigEndian = 0x0160,
    /// MIPS little-endian
    R3000LittleEndian = 0x0162,
    /// MIPS little endian
    R4000 = 0x166,
    /// MIPS little-endian
    R10000 = 0x0168,
    /// Hitachi SH3 little-endian
    SH3 = 0x1a2,
    /// Hitachi SH3 DSP
    SH3DSP = 0x1a3,
    /// SH3E little-endian
    SH3E = 0x01a4,
    /// Hitachi SH4 little-endian
    SH4 = 0x1a6,
    /// Hitachi SH5
    SH5 = 0x1a8,
    /// ARM Thumb/Thumb-2 Little-Endian
    THUMB = 0x1c2,
    /// MIPS little-endian WCE v2
    WCEMIPSV2 = 0x169,
    /// Alpha_AXP
    ALPHA = 0x0184,
    /// ALPHA64
    ALPHA64 = 0x0284,
    /// Infineon
    TRICORE = 0x0520,
    CEF = 0x0CEF,
    CEE = 0xC0EE,
}

impl BinParsable for MachineType {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type MachineType),
            map_opt(le_u16, |raw| Self::from_u16(raw)),
        )(i)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Characteristics: u16 {
        /// Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the
        /// file does not contain base relocations and must therefore be loaded at its preferred
        /// base address. If the base address is not available, the loader reports an error.
        /// The default behavior of the linker is to strip base relocations from
        /// executable (EXE) files.
        const IMAGE_FILE_RELOCS_STRIPPED = 0x0001;

        /// Image only. This indicates that the image file is valid and can be run.
        /// If this flag is not set, it indicates a linker error.
        const IMAGE_FILE_EXECUTABLE_IMAGE = 0x0002;

        /// COFF line numbers have been removed. This flag is deprecated and should be zero.
        const IMAGE_FILE_LINE_NUMS_STRIPPED = 0x0004;

        /// COFF symbol table entries for local symbols have been removed.
        /// This flag is deprecated and should be zero.
        const IMAGE_FILE_LOCAL_SYMS_STRIPPED = 0x0008;

        /// Obsolete. Aggressively trim working set.
        /// This flag is deprecated for Windows 2000 and later and must be zero.
        const IMAGE_FILE_AGGRESSIVE_WS_TRIM = 0x0010;

        /// Application can handle > 2-GB addresses.
        const IMAGE_FILE_LARGE_ADDRESS_AWARE = 0x0020;

        // 0x0040 - This flag is reserved for future use.

        /// Little endian: the least significant bit (LSB) precedes the most significant
        /// bit (MSB) in memory. This flag is deprecated and should be zero.
        const IMAGE_FILE_BYTES_REVERSED_LO = 0x0080;

        /// Machine is based on a 32-bit-word architecture.
        const IMAGE_FILE_32BIT_MACHINE = 0x0100;

        /// Debugging information is removed from the image file.
        const IMAGE_FILE_DEBUG_STRIPPED = 0x0200;

        /// If the image is on removable media, fully load it and copy it to the swap file.
        const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP = 0x0400;

        /// If the image is on network media, fully load it and copy it to the swap file.
        const IMAGE_FILE_NET_RUN_FROM_SWAP = 0x0800;

        /// The image file is a system file, not a user program.
        const IMAGE_FILE_SYSTEM = 0x1000;

        /// The image file is a dynamic-link library (DLL). Such files are considered
        /// executable files for almost all purposes, although they cannot be directly run.
        const IMAGE_FILE_DLL = 0x2000;

        /// The file should be run only on a uniprocessor machine.
        const IMAGE_FILE_UP_SYSTEM_ONLY = 0x4000;

        /// Big endian: the MSB precedes the LSB in memory.
        /// This flag is deprecated and should be zero.
        const IMAGE_FILE_BYTES_REVERSED_HI = 0x8000;
    }
}

impl BinParsable for Characteristics {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type Characteristics),
            map(le_u16, |raw| Self::from_bits_truncate(raw)),
        )(i)
    }
}

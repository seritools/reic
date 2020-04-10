use crate::parsers::BinParsable;

use bitflags::bitflags;
use nameof::name_of;
use nom::{
    combinator::map,
    combinator::map_opt,
    error::context,
    error::ParseError,
    number::complete::{le_u16, le_u32},
    sequence::tuple,
    IResult,
};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, Eq)]
pub struct PE32OptionalHeader {
    /// The address that is relative to the image base of the
    /// beginning-of-data section when it is loaded into memory.
    pub base_of_data: u32,

    /// The next 21 fields are an extension to the COFF optional header format.
    /// They contain additional information that is required by the linker and loader in Windows.
    pub windows_specific: PE32OptionalHeaderWindowsSpecific,
}

impl BinParsable for PE32OptionalHeader {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type PE32OptionalHeader),
            map(
                tuple((le_u32, PE32OptionalHeaderWindowsSpecific::try_parse)),
                |p| Self {
                    base_of_data: p.0,
                    windows_specific: p.1,
                },
            ),
        )(i)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PE32OptionalHeaderWindowsSpecific {
    /// The preferred address of the first byte of image when loaded into memory;
    /// must be a multiple of 64 K.
    ///
    /// The default for DLLs is 0x10000000.
    /// The default for Windows CE EXEs is 0x00010000.
    /// The default for Windows NT, Windows 2000, Windows XP,
    /// Windows 95, Windows 98, and Windows Me is 0x00400000.
    pub image_base: u32,
    /// The alignment (in bytes) of sections when they are loaded into memory.
    /// It must be greater than or equal to FileAlignment.
    /// The default is the page size for the architecture.
    pub section_alignment: u32,
    /// The alignment factor (in bytes) that is used to align the raw data
    /// of sections in the image file.
    ///
    /// The value should be a power of 2 between 512 and 64 K, inclusive. The default is 512.
    ///
    /// If the SectionAlignment is less than the architecture's page size,
    /// then FileAlignment must match SectionAlignment.
    pub file_alignment: u32,
    /// The major version number of the required operating system.
    pub major_operating_system_version: u16,
    /// The minor version number of the required operating system.
    pub minor_operating_system_version: u16,
    /// The major version number of the image.
    pub major_image_version: u16,
    /// The minor version number of the image.
    pub minor_image_version: u16,
    /// The major version number of the subsystem.
    pub major_subsystem_version: u16,
    /// The minor version number of the subsystem.
    pub minor_subsystem_version: u16,
    /// Reserved, must be zero.
    pub win32_version_value: u32,
    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    /// It must be a multiple of SectionAlignment.
    pub size_of_image: u32,
    /// The combined size of an MS-DOS stub, PE header, and section headers
    /// rounded up to a multiple of FileAlignment.
    pub size_of_headers: u32,
    /// The image file checksum.
    ///
    /// The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
    /// The following are checked for validation at load time:
    /// all drivers, any DLL loaded at boot time,
    /// and any DLL that is loaded into a critical Windows process.
    pub check_sum: u32,
    /// The subsystem that is required to run this image.
    /// For more information, see Windows Subsystem.
    pub subsystem: ImageSubsystem,
    /// For more information, see DLL Characteristics later in this specification.
    pub dll_characteristics: DllCharacteristics,
    /// The size of the stack to reserve. Only SizeOfStackCommit is committed;
    /// the rest is made available one page at a time until the reserve size is reached.
    pub size_of_stack_reserve: u32,
    /// The size of the stack to commit.
    pub size_of_stack_commit: u32,
    /// The size of the local heap space to reserve.
    /// Only SizeOfHeapCommit is committed; the rest is made available one page at a time
    /// until the reserve size is reached.
    pub size_of_heap_reserve: u32,
    /// The size of the local heap space to commit.
    pub size_of_heap_commit: u32,
    /// Reserved, must be zero.
    pub loader_flags: u32,
    /// The number of data-directory entries in the remainder of the optional header.
    /// Each describes a location and size.
    pub number_of_rva_and_sizes: u32,
}

impl BinParsable for PE32OptionalHeaderWindowsSpecific {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type PE32OptionalHeaderWindowsSpecific),
            map(
                tuple((
                    le_u32,                        // image_base
                    le_u32,                        // section_alignment
                    le_u32,                        // file_alignment
                    le_u16,                        // major_operating_system_version
                    le_u16,                        // minor_operating_system_version
                    le_u16,                        // major_image_version
                    le_u16,                        // minor_image_version
                    le_u16,                        // major_subsystem_version
                    le_u16,                        // minor_subsystem_version
                    le_u32,                        // win32_version_value
                    le_u32,                        // size_of_image
                    le_u32,                        // size_of_headers
                    le_u32,                        // check_sum
                    ImageSubsystem::try_parse,     // subsystem
                    DllCharacteristics::try_parse, // dll_characteristics
                    le_u32,                        // size_of_stack_reserve
                    le_u32,                        // size_of_stack_commit
                    le_u32,                        // size_of_heap_reserve
                    le_u32,                        // size_of_heap_commit
                    le_u32,                        // loader_flags
                    le_u32,                        // number_of_rva_and_sizes
                )),
                |p| Self {
                    image_base: p.0,
                    section_alignment: p.1,
                    file_alignment: p.2,
                    major_operating_system_version: p.3,
                    minor_operating_system_version: p.4,
                    major_image_version: p.5,
                    minor_image_version: p.6,
                    major_subsystem_version: p.7,
                    minor_subsystem_version: p.8,
                    win32_version_value: p.9,
                    size_of_image: p.10,
                    size_of_headers: p.11,
                    check_sum: p.12,
                    subsystem: p.13,
                    dll_characteristics: p.14,
                    size_of_stack_reserve: p.15,
                    size_of_stack_commit: p.16,
                    size_of_heap_reserve: p.17,
                    size_of_heap_commit: p.18,
                    loader_flags: p.19,
                    number_of_rva_and_sizes: p.20,
                },
            ),
        )(i)
    }
}

#[derive(Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u16)]
pub enum ImageSubsystem {
    /// An unknown subsystem
    Unknown = 0,
    /// Device drivers and native Windows processes
    Native = 1,
    /// The Windows graphical user interface (GUI) subsystem
    WindowsGui = 2,
    /// The Windows character subsystem
    WindowsCui = 3,
    /// The OS/2 character subsystem
    Os2Cui = 5,
    /// The Posix character subsystem
    PosixCui = 7,
    /// Native Win9x driver
    NativeWindows = 8,
    /// Windows CE
    WindowsCeGui = 9,
    /// An Extensible Firmware Interface (EFI) application
    EfiApplication = 10,
    /// An EFI driver with boot services
    EfiBootServiceDriver = 11,
    /// An EFI driver with run-time services
    EfiRuntimeDriver = 12,
    /// An EFI ROM image
    EfiRom = 13,
    /// XBOX
    Xbox = 14,
    /// Windows boot application.
    WindowsBootApplication = 16,
}

impl BinParsable for ImageSubsystem {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type ImageSubsystem),
            map_opt(le_u16, |raw| Self::from_u16(raw)),
        )(i)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct DllCharacteristics: u16 {
        // 0x0001
        // Reserved, must be zero.
        // 0x0002
        // Reserved, must be zero.
        // 0x0004
        // Reserved, must be zero.
        // 0x0008
        // Reserved, must be zero.

        /// Image can handle a high entropy 64-bit virtual address space.
        const HIGH_ENTROPY_VA = 0x0020;
        /// DLL can be relocated at load time.
        const DYNAMIC_BASE = 0x0040;
        /// Code Integrity checks are enforced.
        const FORCE_INTEGRITY = 0x0080;
        /// Image is NX compatible.
        const NX_COMPAT = 0x0100;
        /// Isolation aware, but do not isolate the image.
        const NO_ISOLATION = 0x0200;
        /// Does not use structured exception (SE) handling. No SE handler may be called in this image.
        const NO_SEH = 0x0400;
        /// Do not bind the image.
        const NO_BIND = 0x0800;
        /// Image must execute in an AppContainer.
        const APPCONTAINER = 0x1000;
        /// A WDM driver.
        const WDM_DRIVER = 0x2000;
        /// Image supports Control Flow Guard.
        const GUARD_CF = 0x4000;
        /// Terminal Server aware.
        const TERMINAL_SERVER_AWARE = 0x8000;
    }
}

impl BinParsable for DllCharacteristics {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type DllCharacteristics),
            map(le_u16, |raw| Self::from_bits_truncate(raw)),
        )(i)
    }
}

/// Each data directory gives the address and size of a table or string that Windows uses.
/// These data directory entries are all loaded into memory so that the system can use them at run time.
///
/// The first field, [VirtualAddress], is actually the RVA of the table.
/// The RVA is the address of the table relative to the base address of the image when the
/// table is loaded. The second field gives the size in bytes.
/// The data directories, which form the last part of the optional header,
/// are listed in the following table.
///
/// Note that the number of directories is not fixed. Before looking for a specific directory,
/// check the NumberOfRvaAndSizes field in the optional header.
///
/// Also, do not assume that the RVAs in this table point to the beginning of a section or
/// that the sections that contain specific tables have specific names.
#[derive(Debug, PartialEq, Eq)]
pub struct DataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

impl BinParsable for DataDirectory {
    fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type DataDirectory),
            map(tuple((le_u32, le_u32)), |p| Self {
                virtual_address: p.0,
                size: p.1,
            }),
        )(i)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DataDirectoryType {
    Known(KnownDataDirectoryType),
    Unknown(usize),
}

#[derive(Debug, PartialEq, Eq, FromPrimitive, ToPrimitive, Hash)]
pub enum KnownDataDirectoryType {
    /// Export Directory
    Export = 0,
    /// Import Directory
    Import = 1,
    /// Resource Directory
    Resource = 2,
    /// Exception Directory
    Exception = 3,
    /// Security Directory
    Security = 4,
    /// Base Relocation Table
    Basereloc = 5,
    /// Debug Directory
    Debug = 6,
    /// Architecture Specific Data, COPYRIGHT (X86 usage)
    Architecture = 7,
    /// RVA of GP
    Globalptr = 8,
    /// TLS Directory
    Tls = 9,
    /// Load Configuration Directory
    LoadConfig = 10,
    /// Bound Import Directory in headers
    BoundImport = 11,
    /// Import Address Table
    Iat = 12,
    /// Delay Load Import Descriptors
    DelayImport = 13,
    /// COM Runtime descriptor
    ComDescriptor = 14,
}

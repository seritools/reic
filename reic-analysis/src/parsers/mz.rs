use nameof::name_of;
use nom::{
    bytes::complete::tag,
    combinator::map,
    error::{context, ParseError},
    multi::count,
    number::complete::{le_u16, le_u32},
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct MZHeader {
    /// Bytes on last page of file
    pub e_cblp: u16,
    /// Pages in file
    pub e_cp: u16,
    /// Relocations
    pub e_crlc: u16,
    /// Size of header in paragraphs
    pub e_cparhdr: u16,
    /// Minimum extra paragraphs needed
    pub e_minalloc: u16,
    /// Maximum extra paragraphs needed
    pub e_maxalloc: u16,
    /// Initial (relative) SS value
    pub e_ss: u16,
    /// Initial SP value
    pub e_sp: u16,
    /// Checksum
    pub e_csum: u16,
    /// Initial IP value
    pub e_ip: u16,
    /// Initial (relative) CS value
    pub e_cs: u16,
    /// File address of relocation table
    pub e_lfarlc: u16,
    /// Overlay number
    pub e_ovno: u16,
    /// Reserved words
    pub e_res: [u16; 4],
    /// OEM identifier (for e_oeminfo)
    pub e_oemid: u16,
    /// OEM information; e_oemid specific
    pub e_oeminfo: u16,
    /// Reserved words
    pub e_res2: [u16; 10],
    // File address of new exe header
    pub e_lfanew: u32,
}

impl MZHeader {
    pub fn try_parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        context(
            name_of!(type MZHeader),
            map(
                tuple((
                    tag(b"MZ"),
                    le_u16, // e_cblp,
                    le_u16, // e_cp,
                    le_u16, // e_crlc,
                    le_u16, // e_cparhdr,
                    le_u16, // e_minalloc,
                    le_u16, // e_maxalloc,
                    le_u16, // e_ss,
                    le_u16, // e_sp,
                    le_u16, // e_csum,
                    le_u16, // e_ip,
                    le_u16, // e_cs,
                    le_u16, // e_lfarlc,
                    le_u16, // e_ovno,
                    map(count(le_u16, 4usize), |raw| {
                        let mut data = [0; 4];
                        data.copy_from_slice(&raw[..]);
                        data
                    }), // e_res,
                    le_u16, // e_oemid,
                    le_u16, // e_oeminfo,
                    map(count(le_u16, 10usize), |raw| {
                        let mut data = [0; 10];
                        data.copy_from_slice(&raw[..]);
                        data
                    }), // e_res2,
                    le_u32, // e_lfanew,
                )),
                |p| Self {
                    e_cblp: p.1,
                    e_cp: p.2,
                    e_crlc: p.3,
                    e_cparhdr: p.4,
                    e_minalloc: p.5,
                    e_maxalloc: p.6,
                    e_ss: p.7,
                    e_sp: p.8,
                    e_csum: p.9,
                    e_ip: p.10,
                    e_cs: p.11,
                    e_lfarlc: p.12,
                    e_ovno: p.13,
                    e_res: p.14,
                    e_oemid: p.15,
                    e_oeminfo: p.16,
                    e_res2: p.17,
                    e_lfanew: p.18,
                },
            ),
        )(i)
    }
}

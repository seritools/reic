#![warn(elided_lifetimes_in_paths)]

mod analysis;
mod parsers;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parsers::pe32::PE32Image;
    use crate::parsers::BinParsable;
    use nom::error::{context, VerboseError, VerboseErrorKind};
    use nom::Err::{Error, Failure};
    use nom::Offset;
    use nom::{HexDisplay, IResult};
    use std::cmp::min;

    use zydis::{AddressWidth, Decoder, Formatter, FormatterStyle, MachineMode, OutputBuffer};

    const EXE: &[u8] = include_bytes!("1602.EXE");

    #[test]
    fn it_works() {
        let result = context(
            "pe32",
            parsers::pe32::PE32Image::try_parse::<VerboseError<&[u8]>>,
        )(EXE);

        match result {
            Ok((i, pe32)) => {
                // println!("remaining (len:{:X}):\n{}", i.len(), &i[0..128].to_hex(16));
                // println!("parsed: {:#X?}", pe32);

                let base_addr = pe32.pe32_optional_header.windows_specific.image_base;
                let entry_rva = pe32.coff_optional_header.address_of_entry_point;

                let code_section = pe32
                    .sections
                    .iter()
                    .find(|s| s.header.name == ".text")
                    .expect(".text section");

                let formatter = Formatter::new(FormatterStyle::INTEL).unwrap();
                let decoder = Decoder::new(MachineMode::LEGACY_32, AddressWidth::_32).unwrap();

                // Our actual buffer.
                let mut buffer = [0u8; 256];
                // A wrapped version of the buffer allowing nicer access.
                let mut buffer = OutputBuffer::new(&mut buffer[..]);

                for (instruction, ip) in decoder
                    .instruction_iterator(
                        &code_section.data
                            [(entry_rva - code_section.header.virtual_address) as usize..],
                        (base_addr + entry_rva) as u64,
                    )
                    .take(10)
                {
                    // We use Some(ip) here since we want absolute addressing based on the given
                    // `ip`. If we would want to have relative addressing, we would use
                    // `None` instead.
                    formatter
                        .format_instruction(&instruction, &mut buffer, Some(ip), None)
                        .unwrap();
                    println!("0x{:016X} {}", ip, buffer);
                }
            }
            r => print_err(r),
        }
    }

    fn print_err(result: IResult<&[u8], PE32Image, VerboseError<&[u8]>>) {
        match result {
            Err(Error(e)) | Err(Failure(e)) => {
                for (pos, error_kind) in &e.errors {
                    match error_kind {
                        VerboseErrorKind::Context(c) => {
                            println!("context at 0x{:X}: {}", EXE.offset(pos), c)
                        }
                        VerboseErrorKind::Char(c) => {
                            println!("char expected at 0x{:X}: {}", EXE.offset(pos), c)
                        }
                        VerboseErrorKind::Nom(n) => {
                            println!("nom error at 0x{:X}: {:?}", EXE.offset(pos), n)
                        }
                    }
                }
                if let Some((pos, _)) = e.errors.first() {
                    println!("data at error:\n{}", pos[0..min(128, pos.len())].to_hex(16));
                }
                panic!();
            }
            _ => (),
        }
    }
}

use std::{fmt::Write, fs, path, rc::Rc};

use clap::Parser;
use elf::{abi, endian::LittleEndian, ElfBytes};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArguments {
    program_path: String,
    segment_to_dump: String,
}

trait AdditionalChecks {
    fn is_elf32(&self) -> bool;
    fn is_riscv(&self) -> bool;
}

impl<E: elf::endian::EndianParse> AdditionalChecks for elf::file::FileHeader<E> {
    fn is_elf32(&self) -> bool {
        self.class == elf::file::Class::ELF32
    }

    fn is_riscv(&self) -> bool {
        self.e_machine == abi::EM_RISCV
    }
}

fn main() {
    let arguments = ProgramArguments::parse();

    let path = path::PathBuf::from(&arguments.program_path);

    if !path.exists() {
        panic!("File {} does not exist!", arguments.program_path);
    }

    let file_data = fs::read(path).expect("Unable to open file for reading.");
    let slice = file_data.as_slice();
    let file =
        ElfBytes::<LittleEndian>::minimal_parse(slice).expect("Unable to parse file as ELF.");
    let file_header = file.ehdr;

    if !file_header.is_elf32() {
        panic!("ELF File is not ELF32 format.")
    }

    if !file_header.is_riscv() {
        panic!("ELF File is not RISCV format.")
    }

    let section = file
        .section_header_by_name(arguments.segment_to_dump.as_str())
        .expect("section table should exist.")
        .expect("section should exist.");

    let section_data = file.section_data(&section).unwrap().0;

    let output = section_data
        .chunks(4) /* instructions of 4 bytes */
        .map(|c| c.into_iter().rev().copied().collect::<Rc<[u8]>>()) /* reverse them */
        .map(|c| hex::encode(c))
        .fold(String::default(), |old, new| old + "\n" + &new);

    println!("{}", output);

    // data.chunks_exact(4).for_each(|c| {
    //     let a: Vec<u8> = c.into_iter().rev().copied().collect();
    //     let hex_string = hex::encode(a);
    //     println!("{}",hex_string);
    // });

    // dbg!(data.0);
    // hexdump::hexdump(data);
}

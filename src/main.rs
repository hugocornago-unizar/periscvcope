use std::{fs, path};

use clap::Parser;
use elf::{endian::LittleEndian, ElfBytes};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArguments {
    program_path: String,
    segment_to_dump: String,
}

fn main() {
    let arguments = ProgramArguments::parse();

    let path = path::PathBuf::from(&arguments.program_path);

    if !path.exists() {
        panic!("File {} does not exist!", arguments.program_path);
    }
 
    let file_data = fs::read(path).expect("Unable to open file for reading.");
    let slice = file_data.as_slice();
    let file = ElfBytes::<LittleEndian>::minimal_parse(slice).expect("Unable to parse file as ELF.");

    dbg!(file.ehdr);

}

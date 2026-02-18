use std::{fs, path};

use anyhow::Result;
use periscvcope::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArguments {
    program_path: String,
    segment_to_dump: String,
}

fn main() -> Result<()> {
    let arguments = ProgramArguments::parse();

    let path = path::PathBuf::from(&arguments.program_path);

    if !path.exists() {
        panic!("File {} does not exist!", arguments.program_path);
    }

    let file_data = fs::read(path).expect("Unable to open file for reading.");
    let slice = file_data.as_slice();

    let elf_file = file_parser::ElfFile::from_buffer(slice)?;
    let section = elf_file.find_section_by_name(arguments.segment_to_dump)?;

    dbg!(section);
    Ok(())
}

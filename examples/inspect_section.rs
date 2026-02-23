use std::{fs, path};

use color_eyre::eyre::Result;
use periscvcope::{file_parser::ElfFile, *};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArguments {
    program_path: String,
    segment_to_dump: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let arguments = ProgramArguments::parse();

    let path = path::PathBuf::from(&arguments.program_path);

    if !path.exists() {
        panic!("File {} does not exist!", arguments.program_path);
    }

    let file_data = fs::read(path).expect("Unable to open file for reading.");
    let slice = file_data.as_slice();

    let elf_file = file_parser::ElfFile::from_buffer(slice)?;
    let section = elf_file.find_section_by_name(arguments.segment_to_dump)?;

    let mut memory = elf_file.load_memory(4 * 1024 * 1024);
    let instructions = ElfFile::load_section(section, &mut memory)?;

    let mut sorted: Vec<_> = instructions.iter().collect();
    sorted.sort_by_key(|(addr, _)| *addr);

    for instr in sorted {
        println!("{:08X}: {}", instr.0, instr.1.op());
    }

    Ok(())
}

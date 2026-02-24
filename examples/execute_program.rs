use std::{fs, path};

use color_eyre::eyre::Result;
use periscvcope::machine::Machine;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArguments {
    program_path: String,
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

    let mut machine = Machine::new(slice)?;
    machine.execute_until_loop()?;

    println!("Execution complete.");

    Ok(())
}

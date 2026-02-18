use std::rc::Rc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
}

#[derive(Clone, Debug)]
pub struct Instruction {
    // TODO: utilize custom predefined instructions
    raw_bytes: [u8; 4],
}

impl Instruction {
    pub fn from_bytes(bytes: [u8; 4]) -> Result<Instruction, Error> {
        // TODO: return error if the instruction is invalid
        Ok(Instruction {raw_bytes: bytes})
    }

    pub fn parse_program<'a>(program: impl Into<&'a [u8]>) -> Result<Rc<[Instruction]>, Error> {
        program
            .into()
            .chunks_exact(4 /* 32-bits */)
            .map(|bytes| Instruction::from_bytes(bytes.try_into().expect("chunks_exact ensures 4-byte instructions")))
            .collect()
    }
}

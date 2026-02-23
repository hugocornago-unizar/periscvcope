use std::rc::Rc;
mod formats;
mod parser;

use thiserror::Error;

use crate::instruction::{formats::{InstructionFormat, RType}, parser::Op};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
}

#[derive(Clone, Debug)]
pub struct Instruction {
    op: Op,
    format: InstructionFormat,
    #[allow(dead_code)]
    raw_bytes: [u8; 4],
}

impl Instruction {
    pub fn from_bytes(bytes: [u8; 4]) -> Result<Instruction, Error> {
        let raw = u32::from_le_bytes(bytes);

        let rtype = RType::from(raw);
        let opcode = rtype.opcode().value();
        let funct3 = rtype.funct3().value();
        let funct7 = rtype.funct7().value();

        let op = parser::decode_op(opcode, funct3, funct7)
            .ok_or(Error::UnknownInstruction(opcode.to_string()))?;

        Ok(Instruction { op, format: op.format().decode(raw), raw_bytes: bytes })
    }

    pub fn parse_program<'a>(program: impl Into<&'a [u8]>) -> Result<Vec<Instruction>, Error> {
        program
            .into()
            .chunks_exact(4 /* 32-bits */)
            .map(|bytes| {
                Instruction::from_bytes(
                    bytes
                        .try_into()
                        .expect("chunks_exact ensures 4-byte instructions"),
                )
            })
            .collect()
    }

    pub fn op(&self) -> Op {
        self.op
    }

    pub fn format(&self) -> InstructionFormat {
        self.format
    }
}

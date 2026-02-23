use std::collections::HashMap;

mod executor;

use bilge::prelude::{Integer, u5};

type Memory = Vec<u8>;

use crate::{
    file_parser::{self, ElfFile},
    instruction::{self, Instruction},
};

#[derive(thiserror::Error, Debug)]
pub enum MachineError {
    #[error("AddressError: Tried to access an invalid pc address. pc={0:#X}")]
    AddressError(u32),

    #[error("MemoryError: Tried to access an invalid memory address. addr={0:#X}")]
    MemoryError(u32),

    #[error("Error parsing the file: {0}")]
    ElfError(file_parser::Error),
}

impl From<file_parser::Error> for MachineError {
    fn from(value: file_parser::Error) -> Self {
        Self::ElfError(value)
    }
}

pub struct Machine {
    pc: u32,
    registers: [i32; 32],
    instructions: HashMap<u32, Instruction>,
    memory: Vec<u8>,
}

impl Machine {
    const MEMORY_SIZE: usize = 4 * 1024 * 1024; // 4 MiB
    const _STACK_SIZE: usize = 256 * 1024; // 256 kiB
    const STACK_TOP: i32 = Self::MEMORY_SIZE as i32; // grows backwards
                                                     //
    pub fn new(data: &[u8]) -> Result<Machine, MachineError> {
        let file = ElfFile::from_buffer(data)?;
        let memory = file.load_memory(Self::MEMORY_SIZE);
        let text_section = file.find_section_by_name(".text")?;
        let instructions = ElfFile::load_section(text_section, &memory)?;
        let mut registers = [0i32; 32];

        // set sp
        registers[2] = Self::STACK_TOP & !0xF;


        Ok(Machine {
            pc: file.entry_point(),
            registers,
            instructions,
            memory,
        })
    }

    fn get_register(&self, index: u5) -> i32 {
        self.registers
            .get(index.as_usize())
            .copied()
            .expect("register index not found? check registers array size.")
    }

    fn get_mut_register(&mut self, index: u5) -> &mut i32 {
        self.registers
            .get_mut(index.as_usize())
            .expect("register index not found? check registers array size.")
    }

    pub fn execute_until_loop(&mut self) -> Result<(), MachineError> {
        loop {
            let current_pc = self.pc;
            self.execute_next_instruction()?;
            if self.pc == current_pc { break; };
        }
        Ok(())
    }

    pub fn execute_next_instruction(&mut self) -> Result<(), MachineError> {
        let instr = self
            .instructions
            .get(&self.pc)
            .ok_or(MachineError::AddressError(self.pc))?;
        let format = instr.format();

        use instruction::formats::InstructionFormat::*;
        let new_pc = match format {
            R(rtype) => {
                let rs1_index = rtype.rs1();
                let rs2_index = rtype.rs2();
                let rd_index = rtype.rd();
                let rs1 = self.get_register(rtype.rs1());
                let rs2 = self.get_register(rtype.rs2());
                let op = &instr.op();
                let rd = &mut self.get_mut_register(rtype.rd());

                println!("executing {op}(rd={rd_index} [{rd:#X}], rs1={rs1_index} [{rs1:#X}], rs2={rs2_index} [{rs2:#X}])");
                executor::execute_rtype(op, rd, rs1, rs2)?
            }
            I(itype) => {
                let rs1_index = itype.rs1();
                let rd_index = itype.rd();
                let rs1 = self.get_register(itype.rs1());
                let rd = self
                    .registers
                    .get_mut(itype.rd().value() as usize)
                    .expect("register index not found? check registers array size.");

                let op = &instr.op();
                let imm = format
                    .immediate_value()
                    .expect("I-type should have an immediate value");

                println!("executing {op}(rd={rd_index} [{rd:#X}], rs1={rs1_index} [{rs1:#X}], imm={imm})");
                executor::execute_itype(op, self.pc, rd, rs1, imm, &self.memory)?
            }
            S(stype) => {
                let rs1_index = stype.rs1();
                let rs2_index = stype.rs2();
                let rs1 = self.get_register(stype.rs1());
                let rs2 = self.get_register(stype.rs2());
                let op = &instr.op();

                let imm = format
                    .immediate_value()
                    .expect("S-type should have an immediate value");

                println!("executing {op}(rs1={rs1_index} [{rs1:#X}], rs2={rs2_index} [{rs2:#X}], imm={imm})");
                executor::execute_stype(op, rs1, rs2, imm, &mut self.memory)?
            }
            U(utype) => {
                let op = &instr.op();
                let rd = self
                    .registers
                    .get_mut(utype.rd().value() as usize)
                    .expect("register index not found? check registers array size.");

                let imm = format
                    .immediate_value()
                    .expect("U-type should have an immediate value");

                println!("executing {op}(rd={rd:#X}, imm={imm})");
                executor::execute_utype(op, self.pc, rd, imm)?
            }
            B(btype) => {
                let op = &instr.op();
                let rs1_index = btype.rs1();
                let rs2_index = btype.rs2();
                let rs1 = self.get_register(btype.rs1());
                let rs2 = self.get_register(btype.rs2());

                let imm = format
                    .immediate_value()
                    .expect("S-type should have an immediate value");

                println!("executing {op}(rs1={rs1_index} [{rs1:#X}], rs2={rs2_index} [{rs2:#X}], imm={imm})");
                executor::execute_btype(op, self.pc, rs1, rs2, imm)?
            }
            J(jtype) => {
                let rd_index = jtype.rd();
                let op = &instr.op();
                let rd = self
                    .registers
                    .get_mut(jtype.rd().value() as usize)
                    .expect("register index not found? check registers array size.");

                let imm = format
                    .immediate_value()
                    .expect("J-type should have an immediate value");

                println!("executing {op}(rd={rd_index} [{rd:#X}], imm={imm})");
                executor::execute_jtype(op, self.pc, rd, imm)?
            }
        };

        // hardwire x0 to 0.
        self.registers[0] = 0;

        if let Some(pc) = new_pc {
            println!("jumping to {pc:#X}");
            self.pc = pc;
        } else {
            self.pc += 4;
        }

        Ok(())
    }
}

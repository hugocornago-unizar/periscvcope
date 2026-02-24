use bilge::prelude::{Integer, u5};

use crate::{
    instruction::definitions::Op,
    machine::{MachineError, Memory},
};

pub(crate) fn execute_rtype(
    op: &Op,
    rd: &mut i32,
    rs1: i32,
    rs2: i32,
) -> Result<Option<u32>, MachineError> {
    Ok(match op {
        Op::add => {
            *rd = rs1 + rs2;
            None
        }
        Op::sub => {
            *rd = rs1 - rs2;
            None
        }
        Op::xor => {
            *rd = rs1 ^ rs2;
            None
        }
        Op::or => {
            *rd = rs1 | rs2;
            None
        }
        Op::and => {
            *rd = rs1 & rs2;
            None
        }
        Op::sll => {
            *rd = rs1 << rs2;
            None
        }
        Op::srl => {
            *rd = (rs1 as u32 >> rs2) as i32;
            None
        }
        Op::sra => {
            *rd = rs1 >> rs2;
            None
        }
        Op::slt => {
            *rd = if rs1 < rs2 { 1 } else { 0 };
            None
        }
        Op::sltu => {
            *rd = if (rs1 as u32) < (rs2 as u32) { 1 } else { 0 };
            None
        }
        _ => panic!("executing operation {} as a R-type.", op),
    })
}

pub(crate) fn execute_itype(
    op: &Op,
    pc: u32,
    rd: &mut i32,
    rs1: i32,
    imm: i32,
    memory: &Memory,
) -> Result<Option<u32>, MachineError> {
    Ok(match op {
        Op::addi => {
            *rd = rs1 + imm;
            None
        }
        Op::xori => {
            *rd = rs1 ^ imm;
            None
        }
        Op::ori => {
            *rd = rs1 | imm;
            None
        }
        Op::andi => {
            *rd = rs1 & imm;
            None
        }
        Op::slli => {
            let reduced_imm = u5::extract_u32(imm as u32, 0).as_i32();
            *rd = rs1 << reduced_imm;
            None
        }
        Op::srli => {
            let reduced_imm = u5::extract_u32(imm as u32, 0).as_i32();
            *rd = ((rs1 as u32) >> reduced_imm) as i32;
            None
        }
        Op::srai => {
            let reduced_imm = u5::extract_u32(imm as u32, 0).as_i32();
            *rd = rs1 >> reduced_imm;
            None
        }
        Op::slti => {
            *rd = if rs1 < imm { 1 } else { 0 };
            None
        }
        Op::sltiu => {
            *rd = if (rs1 as u32) < (imm as u32) { 1 } else { 0 };
            None
        }
        Op::lb => {
            let addr = rs1 + imm;
            let byte = memory
                .get(addr as usize)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            *rd = *byte as i8 as i32;
            None
        }
        Op::lh => {
            let addr = rs1 + imm;
            let b1 = memory
                .get(addr as usize)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            let b2 = memory
                .get(addr as usize + 1)
                .ok_or(MachineError::MemoryError(addr as u32 + 1))?;
            let value = i16::from_le_bytes([*b1, *b2]);
            *rd = value as i32;
            None
        }
        Op::lw => {
            let addr = rs1 + imm;
            let b1 = memory
                .get(addr as usize)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            let b2 = memory
                .get(addr as usize + 1)
                .ok_or(MachineError::MemoryError(addr as u32 + 1))?;
            let b3 = memory
                .get(addr as usize + 2)
                .ok_or(MachineError::MemoryError(addr as u32 + 2))?;
            let b4 = memory
                .get(addr as usize + 3)
                .ok_or(MachineError::MemoryError(addr as u32 + 3))?;
            let value = i32::from_le_bytes([*b1, *b2, *b3, *b4]);
            *rd = value;
            None
        }
        Op::lbu => {
            let addr = rs1 + imm;
            let byte = memory
                .get(addr as usize)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            *rd = *byte as i32;
            None
        }
        Op::lhu => {
            let addr = rs1 + imm;
            let b1 = memory
                .get(addr as usize)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            let b2 = memory
                .get(addr as usize + 1)
                .ok_or(MachineError::MemoryError(addr as u32 + 1))?;
            let value = u16::from_le_bytes([*b1, *b2]);
            *rd = value as i32;
            None
        }
        Op::jalr => {
            *rd = (pc + 4) as i32;
            let addr = rs1 + imm;
            Some(addr as u32)
        }
        Op::ecall => unimplemented!("operation ecall is not implemented"),
        Op::ebreak => unimplemented!("operation ebreak is not implemented"),
        _ => panic!("executing operation {} as a I-type.", op),
    })
}

pub(crate) fn execute_stype(
    op: &Op,
    rs1: i32,
    rs2: i32,
    imm: i32,
    memory: &mut Memory,
) -> Result<Option<u32>, MachineError> {
    Ok(match op {
        Op::sb => {
            let addr = rs1 + imm;
            let value = memory
                .get_mut(addr as usize)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            *value = rs2 as u8;
            None
        }
        Op::sh => {
            let addr = (rs1 + imm) as usize;
            let slice = memory
                .get_mut(addr..addr + 2)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            slice.copy_from_slice(&(rs2 as i16).to_le_bytes());

            None
        }
        Op::sw => {
            let addr = (rs1 + imm) as usize;
            let slice = memory
                .get_mut(addr..addr + 4)
                .ok_or(MachineError::MemoryError(addr as u32))?;
            slice.copy_from_slice(&rs2.to_le_bytes());

            None
        }
        _ => panic!("executing operation {} as a S-type.", op),
    })
}

pub(crate) fn execute_btype(
    op: &Op,
    pc: u32,
    rs1: i32,
    rs2: i32,
    imm: i32,
) -> Result<Option<u32>, MachineError> {
    Ok(match op {
        Op::beq => {
            if rs1 == rs2 {
                let addr = pc as i32 + imm;
                Some(addr as u32)
            } else {
                None
            }
        }
        Op::bne => {
            if rs1 != rs2 {
                let addr = pc as i32 + imm;
                Some(addr as u32)
            } else {
                None
            }
        }
        Op::blt => {
            if rs1 < rs2 {
                let addr = pc as i32 + imm;
                Some(addr as u32)
            } else {
                None
            }
        }
        Op::bge => {
            if rs1 >= rs2 {
                let addr = pc as i32 + imm;
                Some(addr as u32)
            } else {
                None
            }
        }
        Op::bltu => {
            if (rs1 as u32) < (rs2 as u32) {
                let addr = pc as i32 + imm;
                Some(addr as u32)
            } else {
                None
            }
        }
        Op::bgeu => {
            if (rs1 as u32) >= (rs2 as u32) {
                let addr = pc as i32 + imm;
                Some(addr as u32)
            } else {
                None
            }
        }
        _ => panic!("executing operation {} as a B-type.", op),
    })
}

pub(crate) fn execute_jtype(
    op: &Op,
    pc: u32,
    rd: &mut i32,
    imm: i32,
) -> Result<Option<u32>, MachineError> {
    Ok(match op {
        Op::jal => {
            *rd = (pc + 4) as i32;
            let addr = pc as i32 + imm;
            Some(addr as u32)
        }
        _ => panic!("executing operation {} as a B-type.", op),
    })
}

pub(crate) fn execute_utype(
    op: &Op,
    pc: u32,
    rd: &mut i32,
    imm: i32,
) -> Result<Option<u32>, MachineError> {
    Ok(match op {
        Op::lui => {
            *rd = imm << 12;
            None
        }
        Op::auipc => {
            *rd = (pc as i32) + (imm << 12);
            None
        }
        _ => panic!("executing operation {} as a U-type.", op),
    })
}

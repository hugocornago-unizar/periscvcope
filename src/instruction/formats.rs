use bilge::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format { R, I, S, B, U, J }

impl Format {
    pub fn decode(self, raw: u32) -> InstructionFormat {
        match self {
            Format::R => InstructionFormat::R(RType::from(raw)),
            Format::I => InstructionFormat::I(IType::from(raw)),
            Format::S => InstructionFormat::S(SType::from(raw)),
            Format::B => InstructionFormat::B(BType::from(raw)),
            Format::U => InstructionFormat::U(UType::from(raw)),
            Format::J => InstructionFormat::J(JType::from(raw)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionFormat {
    R(RType),
    I(IType),
    S(SType),
    U(UType),
    B(BType),
    J(JType),
}

fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value as i32) << shift) >> shift
}

impl InstructionFormat {
    pub fn opcode(&self) -> u7 {
        match self {
            InstructionFormat::R(rtype) => rtype.opcode(),
            InstructionFormat::I(itype) => itype.opcode(),
            InstructionFormat::S(stype) => stype.opcode(),
            InstructionFormat::U(utype) => utype.opcode(),
            InstructionFormat::B(btype) => btype.opcode(),
            InstructionFormat::J(jtype) => jtype.opcode(),
        }
    }
    pub fn rd(&self) -> Option<u5> {
        match self {
            InstructionFormat::R(rtype) => Some(rtype.rd()),
            InstructionFormat::I(itype) => Some(itype.rd()),
            InstructionFormat::S(..) => None,
            InstructionFormat::U(utype) => Some(utype.rd()),
            InstructionFormat::B(..) => None,
            InstructionFormat::J(jtype) => Some(jtype.rd()),
        }
    }

    pub fn rs1(&self) -> Option<u5> {
        match self {
            InstructionFormat::R(rtype) => Some(rtype.rs1()),
            InstructionFormat::I(itype) => Some(itype.rs1()),
            InstructionFormat::S(stype) => Some(stype.rs1()),
            InstructionFormat::U(..) => None,
            InstructionFormat::B(btype) => Some(btype.rs1()),
            InstructionFormat::J(..) => None,
        }
    }

    pub fn rs2(&self) -> Option<u5> {
        match self {
            InstructionFormat::R(rtype) => Some(rtype.rs2()),
            InstructionFormat::I(..) => None,
            InstructionFormat::S(stype) => Some(stype.rs2()),
            InstructionFormat::U(..) => None,
            InstructionFormat::B(btype) => Some(btype.rs2()),
            InstructionFormat::J(..) => None,
        }
    }

    // TODO: unit testing
    pub fn immediate_value(&self) -> Option<i32> {
        match self {
            InstructionFormat::R(..) => None,
            InstructionFormat::I(itype) => Some(sign_extend(itype.imm().as_u32(), 12)),
            InstructionFormat::S(stype) => Some({
                let value =   stype.imm1().as_u32()
                            | stype.imm2().as_u32() << 6;

                sign_extend(value, 12)
            }),
            InstructionFormat::U(utype) => Some({
                utype.imm().as_i32() << 12
            }),
            InstructionFormat::B(btype) => Some({
                let value =   btype.imm1().as_u32() << 11
                            | btype.imm2().as_u32() << 1
                            | btype.imm3().as_u32() << 5
                            | btype.imm4().as_u32() << 12;

                sign_extend(value, 13)
            }),
            InstructionFormat::J(jtype) => Some({
                let value =   jtype.imm1().as_u32() << 12
                            | jtype.imm2().as_u32() << 11
                            | jtype.imm3().as_u32() << 1
                            | jtype.imm4().as_u32() << 20;

                sign_extend(value, 21)
            }),
        }
    }
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, Clone, Copy)]
pub struct RType { 
    pub opcode: u7,
    rd: u5,
    pub funct3: u3,
    rs1: u5,
    rs2: u5,
    pub funct7: u7
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, Clone, Copy)]
pub struct IType { 
    opcode: u7,
    rd: u5,
    funct3: u3,
    rs1: u5,
    imm: u12
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, Clone, Copy)]
pub struct SType { 
    opcode: u7,
    imm1: u5,
    funct3: u3,
    rs1: u5,
    rs2: u5,
    imm2: u7
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, Clone, Copy)]
pub struct UType { 
    opcode: u7,
    rd: u5,
    imm: u20
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, Clone, Copy)]
pub struct BType { 
    opcode: u7,
    imm1: u1,
    imm2: u4,
    funct3: u3,
    rs1: u5,
    rs2: u5,
    imm3: u6,
    imm4: u1,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, Clone, Copy)]
pub struct JType { 
    opcode: u7,
    rd: u5,
    imm1: u8,
    imm2: u1,
    imm3: u10,
    imm4: u1
}

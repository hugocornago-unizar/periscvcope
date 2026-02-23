#![allow(dead_code)]
use crate::instruction::formats::*;

macro_rules! define_instructions {
    (
        $(
            $variant:ident : $format:ident {
                opcode: $opcode:expr
                $(, funct3: $funct3:expr)?
                $(, funct7: $funct7:expr)?
            }
        ),*
        $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Op {
            $($variant),*
        }

        impl Op {
            pub fn format(self) -> Format {
                match self {
                    $(Op::$variant => Format::$format),*
                }
            }
        }

        pub fn decode_op(opcode: u8, funct3: u8, funct7: u8) -> Option<Op> {
            match (opcode, funct3, funct7) {
                $(
                    (
                        $opcode,
                        define_instructions!(@field $($funct3)?),
                        define_instructions!(@field $($funct7)?)
                    ) => Some(Op::$variant),
                )*
                _ => None,
            }
        }
    };

    (@field $val:expr) => { $val };
    (@field) => { _ };
}

define_instructions! {
    Add  : R { opcode: 0b0110011, funct3: 0x0, funct7: 0x00 },
    Sub  : R { opcode: 0b0110011, funct3: 0x0, funct7: 0x20 },
    Sll  : R { opcode: 0b0110011, funct3: 0x1, funct7: 0x00 },
    Slt  : R { opcode: 0b0110011, funct3: 0x2, funct7: 0x00 },
    Sltu : R { opcode: 0b0110011, funct3: 0x3, funct7: 0x00 },
    Xor  : R { opcode: 0b0110011, funct3: 0x4, funct7: 0x00 },
    Srl  : R { opcode: 0b0110011, funct3: 0x5, funct7: 0x00 },
    Sra  : R { opcode: 0b0110011, funct3: 0x5, funct7: 0x20 },
    Or   : R { opcode: 0b0110011, funct3: 0x6, funct7: 0x00 },
    And  : R { opcode: 0b0110011, funct3: 0x7, funct7: 0x00 },

    Slli  : I { opcode: 0b0010011, funct3: 0x1, funct7: 0x00 },
    Srli  : I { opcode: 0b0010011, funct3: 0x5, funct7: 0x00 },
    Srai  : I { opcode: 0b0010011, funct3: 0x5, funct7: 0x20 },

    Addi  : I { opcode: 0b0010011, funct3: 0x0 },
    Slti  : I { opcode: 0b0010011, funct3: 0x2 },
    Sltiu : I { opcode: 0b0010011, funct3: 0x3 },
    Xori  : I { opcode: 0b0010011, funct3: 0x4 },
    Ori   : I { opcode: 0b0010011, funct3: 0x6 },
    Andi  : I { opcode: 0b0010011, funct3: 0x7 },

    Lb  : I { opcode: 0b0000011, funct3: 0x0 },
    Lh  : I { opcode: 0b0000011, funct3: 0x1 },
    Lw  : I { opcode: 0b0000011, funct3: 0x2 },
    Lbu : I { opcode: 0b0000011, funct3: 0x4 },
    Lhu : I { opcode: 0b0000011, funct3: 0x5 },

    Sb : S { opcode: 0b0100011, funct3: 0x0 },
    Sh : S { opcode: 0b0100011, funct3: 0x1 },
    Sw : S { opcode: 0b0100011, funct3: 0x2 },

    Beq  : B { opcode: 0b1100011, funct3: 0x0 },
    Bne  : B { opcode: 0b1100011, funct3: 0x1 },
    Blt  : B { opcode: 0b1100011, funct3: 0x4 },
    Bge  : B { opcode: 0b1100011, funct3: 0x5 },
    Bltu : B { opcode: 0b1100011, funct3: 0x6 },
    Bgeu : B { opcode: 0b1100011, funct3: 0x7 },

    Lui   : U { opcode: 0b0110111 },
    Auipc : U { opcode: 0b0010111 },

    Jal  : J { opcode: 0b1101111 },

    Jalr : I { opcode: 0b1100111, funct3: 0x0 },
}

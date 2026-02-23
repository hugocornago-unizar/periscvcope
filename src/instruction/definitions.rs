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
        #[allow(non_camel_case_types)]
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

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/* From RISCV_CARD.pdf */
define_instructions! {
    add  : R { opcode: 0b0110011, funct3: 0x0, funct7: 0x00 },
    sub  : R { opcode: 0b0110011, funct3: 0x0, funct7: 0x20 },
    xor  : R { opcode: 0b0110011, funct3: 0x4, funct7: 0x00 },
    or   : R { opcode: 0b0110011, funct3: 0x6, funct7: 0x00 },
    and  : R { opcode: 0b0110011, funct3: 0x7, funct7: 0x00 },
    sll  : R { opcode: 0b0110011, funct3: 0x1, funct7: 0x00 },
    srl  : R { opcode: 0b0110011, funct3: 0x5, funct7: 0x00 },
    sra  : R { opcode: 0b0110011, funct3: 0x5, funct7: 0x20 },
    slt  : R { opcode: 0b0110011, funct3: 0x2, funct7: 0x00 },
    sltu : R { opcode: 0b0110011, funct3: 0x3, funct7: 0x00 },

    addi  : I { opcode: 0b0010011, funct3: 0x0 },
    xori  : I { opcode: 0b0010011, funct3: 0x4 },
    ori   : I { opcode: 0b0010011, funct3: 0x6 },
    andi  : I { opcode: 0b0010011, funct3: 0x7 },
    slli  : I { opcode: 0b0010011, funct3: 0x1, funct7: 0x00 },
    srli  : I { opcode: 0b0010011, funct3: 0x5, funct7: 0x00 },
    srai  : I { opcode: 0b0010011, funct3: 0x5, funct7: 0x20 },
    slti  : I { opcode: 0b0010011, funct3: 0x2 },
    sltiu : I { opcode: 0b0010011, funct3: 0x3 },

    lb  : I { opcode: 0b0000011, funct3: 0x0 },
    lh  : I { opcode: 0b0000011, funct3: 0x1 },
    lw  : I { opcode: 0b0000011, funct3: 0x2 },
    lbu : I { opcode: 0b0000011, funct3: 0x4 },
    lhu : I { opcode: 0b0000011, funct3: 0x5 },

    sb : S { opcode: 0b0100011, funct3: 0x0 },
    sh : S { opcode: 0b0100011, funct3: 0x1 },
    sw : S { opcode: 0b0100011, funct3: 0x2 },

    beq  : B { opcode: 0b1100011, funct3: 0x0 },
    bne  : B { opcode: 0b1100011, funct3: 0x1 },
    blt  : B { opcode: 0b1100011, funct3: 0x4 },
    bge  : B { opcode: 0b1100011, funct3: 0x5 },
    bltu : B { opcode: 0b1100011, funct3: 0x6 },
    bgeu : B { opcode: 0b1100011, funct3: 0x7 },

    lui   : U { opcode: 0b0110111 },
    auipc : U { opcode: 0b0010111 },

    jal  : J { opcode: 0b1101111 },

    jalr : I { opcode: 0b1100111, funct3: 0x0 },

    ecall : I { opcode: 0b1110011, funct3: 0x0, funct7: 0x0 },
    ebreak : I { opcode: 0b1110011, funct3: 0x0, funct7: 0x1 },
}

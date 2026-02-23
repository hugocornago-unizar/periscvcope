use std::rc::Rc;

use elf::{ElfBytes, abi, endian::LittleEndian};
use thiserror::Error;

use crate::instruction::Instruction;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ELF file supplied is not built for RISCV.")]
    NotRISCV(),

    #[error("ELF file supplied is not ELF32.")]
    NotELF32(),

    #[error("ELF file does not contain a section header.")]
    NoSectionHeader(),

    #[error("ELF file does not contain a segment header.")]
    NoSegmentHeader(),

    #[error("Section {0} does not exists.")]
    SectionNotFound(String),

    #[error("Error while parsing ELF file: {0}")]
    ParseError(elf::ParseError),

    #[error("Error while parsing instruction: {0}")]
    InstructionParseError(crate::instruction::Error),

    #[error("Unknown error {0}")]
    #[allow(dead_code)]
    Unknown(String),
}

impl From<elf::ParseError> for Error {
    fn from(error: elf::ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl From<crate::instruction::Error> for Error {
    fn from(error: crate::instruction::Error) -> Self {
        Self::InstructionParseError(error)
    }
}

pub struct ElfFile<'a> {
    parser: ElfBytes<'a, LittleEndian>,
    sections: elf::section::SectionHeaderTable<'a, LittleEndian>,
    segments: elf::segment::SegmentTable<'a, LittleEndian>,
}

#[allow(dead_code)] // TODO: remove
impl<'a> ElfFile<'a> {
    /* Creates a new ElfFile from a ELF32 RISCV file buffer */
    pub fn from_buffer(buffer: &'a [u8]) -> Result<Self, Error> {
        let parser = ElfBytes::<LittleEndian>::minimal_parse(buffer)?;
        let sections = parser.section_headers().ok_or(Error::NoSectionHeader())?;
        let segments = parser.segments().ok_or(Error::NoSegmentHeader())?;

        let file = Self {
            parser,
            sections,
            segments,
        };

        if !file.check_elf32() {
            return Err(Error::NotELF32());
        }

        if !file.check_riscv() {
            return Err(Error::NotRISCV());
        }

        Ok(file)
    }

    pub fn find_section_by_name(
        &self,
        segment_name: impl Into<String>,
    ) -> Result<Vec<Instruction>, Error> {
        let segment_name = segment_name.into();
        let section = self
            .parser
            .section_header_by_name(&segment_name)
            .expect("Already checked the existance for section headers.")
            .ok_or(Error::SectionNotFound(segment_name))?;

        let program = self.parser.section_data(&section)?.0;

        Ok(Instruction::parse_program(program)?)
    }

    /* Checks if the ElfFile is built for RISCV. */
    fn check_riscv(&self) -> bool {
        self.parser.ehdr.e_machine == abi::EM_RISCV
    }

    /* Checks if the ElfFile is an ELF32 program. */
    fn check_elf32(&self) -> bool {
        self.parser.ehdr.class == elf::file::Class::ELF32
    }
}

use std::collections::HashMap;

use elf::{ElfBytes, abi, endian::LittleEndian, section::SectionHeader};
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
    ParseError(#[from] elf::ParseError),

    #[error("Error while parsing instruction: {0}")]
    InstructionParseError(#[from] crate::instruction::Error),

    #[error("Unknown error {0}")]
    #[allow(dead_code)]
    Unknown(String),
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

    pub fn entry_point(&self) -> u32 {
        self.parser.ehdr.e_entry as u32
    }

    pub fn load_memory(&self, size: usize) -> Vec<u8> {
        let mut memory = vec![0u8; size];

        self.segments
            .iter()
            .filter(|phdr| phdr.p_type == abi::PT_LOAD)
            .map(|phdr| {
                (
                    phdr.p_vaddr,
                    self.parser
                        .segment_data(&phdr)
                        .expect("IntegerOverflow error"),
                )
            })
            .for_each(|(addr, data)| {
                let start = addr as usize;
                let end = start + data.len();
                memory[start..end].copy_from_slice(data);
            });

        memory
    }

    pub fn load_section(
        section: SectionHeader,
        memory: &[u8],
    ) -> Result<HashMap<u32, Instruction>, Error> {
        let start = section.sh_addr as usize;
        let size = section.sh_size as usize;
        let data = &memory[start..start + size];

        data.chunks_exact(4)
            .enumerate()
            .map(|(i, bytes)| {
                let addr = start + (i * 4);
                let instr = Instruction::from_bytes(bytes.try_into().expect("it should work"))?;

                Ok((addr as u32, instr))
            })
            .collect::<Result<_, _>>()
    }

    pub fn find_section_by_name(
        &self,
        segment_name: impl Into<String>,
    ) -> Result<SectionHeader, Error> {
        let segment_name = segment_name.into();
        let section = self
            .parser
            .section_header_by_name(&segment_name)
            .expect("Already checked the existance for section headers.")
            .ok_or(Error::SectionNotFound(segment_name))?;

        Ok(section)
    }

    /* Checks if the ElfFile is built for RISCV. */
    fn check_riscv(&self) -> bool {
        self.parser.ehdr.e_machine == abi::EM_RISCV
    }

    /* Checks if the ElfFile is an ELF32 program. */
    fn check_elf32(&self) -> bool {
        self.parser.ehdr.class == elf::file::Class::ELF32
    }

    pub fn sections(&self) -> elf::parse::ParsingTable<'a, LittleEndian, SectionHeader> {
        self.sections
    }
}

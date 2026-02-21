//! Estructuras de cabecera ELF32

use core::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

impl Elf32_Ehdr {
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() < mem::size_of::<Self>() {
            return None;
        }
        Some(unsafe { &*(bytes.as_ptr() as *const Self) })
    }
    
    pub fn is_valid(&self) -> bool {
        self.e_ident[0] == super::types::ELFMAG0 &&
        self.e_ident[1] == super::types::ELFMAG1 &&
        self.e_ident[2] == super::types::ELFMAG2 &&
        self.e_ident[3] == super::types::ELFMAG3
    }
}

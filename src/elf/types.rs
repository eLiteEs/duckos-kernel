//! Constantes y tipos para ELF

// Tipos de segmento (Program Header)
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;

// Flags de segmento
pub const PF_X: u32 = 1; // Ejecutable
pub const PF_W: u32 = 2; // Escribible  
pub const PF_R: u32 = 4; // Legible

// Identificación ELF (e_ident indices)
pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI: usize = 7;
pub const EI_ABIVERSION: usize = 8;

// Clases ELF
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;

// Magic numbers
pub const ELFMAG0: u8 = 0x7F;
pub const ELFMAG1: u8 = b'E';
pub const ELFMAG2: u8 = b'L';
pub const ELFMAG3: u8 = b'F';

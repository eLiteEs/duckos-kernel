//! Cargador de archivos ELF

use super::header::{Elf32_Ehdr, Elf32_Phdr};
use super::types::PT_LOAD;
use crate::syscall::Syscalls;
use crate::println; // O tu macro de print

pub struct ElfLoader;

impl ElfLoader {
    pub fn load_and_execute(file: &[u8], syscalls: &mut dyn Syscalls) -> Result<(), &'static str> {
        let ehdr = Elf32_Ehdr::from_bytes(file).ok_or("Archivo demasiado pequeño")?;
    
        if !ehdr.is_valid() {
            return Err("No es un archivo ELF válido");
        }
    
        // Mostrar info de debug
        crate::println!("ELF: entry point = 0x{:x}", ehdr.e_entry);
        crate::println!("ELF: {} program headers", ehdr.e_phnum);
    
        Self::load_segments(file, ehdr)?;
    
        // IMPORTANTE: Asegurar que la firma de la función coincide
        type EntryFn = extern "C" fn(&mut dyn Syscalls);
        let entry_fn: EntryFn = unsafe { core::mem::transmute(ehdr.e_entry as usize) };
    
        crate::println!("Ejecutando punto de entrada...");
        entry_fn(syscalls);
    
        Ok(())
    }

    fn load_segments(file: &[u8], ehdr: &Elf32_Ehdr) -> Result<(), &'static str> {
        let phoff = ehdr.e_phoff as usize;
        let phentsize = ehdr.e_phentsize as usize;
        let phnum = ehdr.e_phnum as usize;
        
        // Verificar que los program headers están dentro del archivo
        if file.len() < phoff + phnum * phentsize {
            return Err("Program headers fuera del archivo");
        }
        
        for i in 0..phnum {
            let phdr_ptr = (file.as_ptr() as usize + phoff + i * phentsize) as *const Elf32_Phdr;
            let phdr = unsafe { &*phdr_ptr };
            
            if phdr.p_type != PT_LOAD {
                continue;
            }
            
            Self::load_segment(file, phdr)?;
        }
        
        Ok(())
    }
    
    fn load_segment(file: &[u8], phdr: &Elf32_Phdr) -> Result<(), &'static str> {
        let vaddr = phdr.p_vaddr as usize;
        let offset = phdr.p_offset as usize;
        let filesz = phdr.p_filesz as usize;
        let memsz = phdr.p_memsz as usize;
        
        // Verificar que el segmento cabe en el archivo
        if file.len() < offset + filesz {
            return Err("Segmento fuera del archivo");
        }
        
        // Destino en memoria (¡CUIDADO! Esto asume que podemos escribir en vaddr)
        // En un kernel real, deberías mapear esta memoria primero
        let dest = vaddr as *mut u8;
        
        unsafe {
            // Copiar datos
            let src = &file[offset..offset + filesz];
            core::ptr::copy_nonoverlapping(src.as_ptr(), dest, filesz);
            
            // Limpiar BSS
            if memsz > filesz {
                let bss_start = dest.add(filesz);
                core::ptr::write_bytes(bss_start, 0, memsz - filesz);
            }
        }
        
        Ok(())
    }
}

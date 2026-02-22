// src/elf/loader.rs
use super::header::{Elf32_Ehdr, Elf32_Phdr};
use super::types::PT_LOAD;
use crate::syscall::Syscalls;
use crate::memory::{map_range, FRAME_ALLOCATOR};
use x86_64::{VirtAddr, structures::paging::PageTableFlags};

pub struct ElfLoader;

impl ElfLoader {
    pub fn load_and_execute(file: &[u8], syscalls: &mut dyn Syscalls) -> Result<(), &'static str> {
        crate::println!("📦 Cargando ELF...");
        
        let ehdr = Elf32_Ehdr::from_bytes(file).ok_or("Archivo demasiado pequeño")?;
        crate::println!("  Magic: {:02x?}", &ehdr.e_ident[0..4]);
        crate::println!("  Entry: 0x{:x}", ehdr.e_entry);
        crate::println!("  PHDRs: {}", ehdr.e_phnum);
        
        if !ehdr.is_valid() {
            return Err("No es un archivo ELF válido");
        }
        
        // Cargar segmentos
        Self::load_segments(file, ehdr)?;
        crate::println!("✅ Segmentos cargados");
        
        crate::println!("🚀 Saltando a entry point: 0x{:x}", ehdr.e_entry);
        type EntryFn = extern "C" fn(&mut dyn Syscalls);
        let entry_fn: EntryFn = unsafe { core::mem::transmute(ehdr.e_entry as usize) };
        
        entry_fn(syscalls);
        
        Ok(())
    }
    
    fn load_segments(file: &[u8], ehdr: &Elf32_Ehdr) -> Result<(), &'static str> {
        let phoff = ehdr.e_phoff as usize;
        let phentsize = ehdr.e_phentsize as usize;
        let phnum = ehdr.e_phnum as usize;
        
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
        
        crate::println!("Cargando segmento en 0x{:x} ({} bytes)", vaddr, memsz);
        
        // Calcular páginas necesarias
        let page_count = (memsz + 4095) / 4096;
        let virt_start = VirtAddr::new(vaddr as u64);
        
        // Mapear memoria
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        map_range(virt_start, page_count as u64, flags)
            .map_err(|_| "Error al mapear memoria")?;
        
        // Destino
        let dest = virt_start.as_mut_ptr();
        
        unsafe {
            // Copiar datos
            if filesz > 0 {
                if file.len() < offset + filesz {
                    return Err("Segmento fuera del archivo");
                }
                let src = &file[offset..offset + filesz];
                crate::println!("Copiando {} bytes a 0x{:x}", filesz, vaddr);
                core::ptr::copy_nonoverlapping(src.as_ptr(), dest, filesz);
            }
            
            // Limpiar BSS
            if memsz > filesz {
                let bss_start = dest.add(filesz);
                let bss_size = memsz - filesz;
                crate::println!("Limpiando BSS: {} bytes en 0x{:x}", bss_size, vaddr + filesz);
                core::ptr::write_bytes(bss_start, 0, bss_size);
            }
        }
        
        Ok(())
    }
}

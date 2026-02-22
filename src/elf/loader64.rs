// src/elf/loader64.rs
use super::header64::{Elf64_Ehdr, Elf64_Phdr};
use super::types::PT_LOAD;
use crate::syscall::Syscalls;
use crate::memory::map_range;
use x86_64::{VirtAddr, structures::paging::PageTableFlags};

pub struct ElfLoader;

impl ElfLoader {
    pub fn load_and_execute(file: &[u8], syscalls: &mut dyn Syscalls) -> Result<(), &'static str> {
        crate::println!("📦 Cargando ELF64...");
        crate::println!("Tamaño del archivo: {} bytes", file.len());
        
        let ehdr = Elf64_Ehdr::from_bytes(file).ok_or("Archivo demasiado pequeño")?;
        crate::println!("  Magic: {:02x?}", &ehdr.e_ident[0..4]);
        crate::println!("  Entry: 0x{:x}", ehdr.e_entry);
        crate::println!("  PHDRs: {}", ehdr.e_phnum);
        crate::println!("  PHDR offset: 0x{:x}", ehdr.e_phoff);
        crate::println!("  PHDR entry size: {} bytes", ehdr.e_phentsize);
        
        if !ehdr.is_valid() {
            return Err("No es un archivo ELF válido");
        }
        
        if ehdr.e_phnum == 0 {
            crate::println!("⚠️  ELF sin program headers");
            return Err("ELF sin segmentos cargables");
        }
        
        Self::load_segments(file, ehdr)?;
        crate::println!("✅ Segmentos cargados");
        
        crate::println!("🚀 Saltando a entry point: 0x{:x}", ehdr.e_entry);
        type EntryFn = extern "C" fn(&mut dyn Syscalls);
        let entry_fn: EntryFn = unsafe { core::mem::transmute(ehdr.e_entry as usize) };
        
        entry_fn(syscalls);
        
        Ok(())
    }
    
    fn load_segments(file: &[u8], ehdr: &Elf64_Ehdr) -> Result<(), &'static str> {
        let phoff = ehdr.e_phoff as usize;
        let phentsize = ehdr.e_phentsize as usize;
        let phnum = ehdr.e_phnum as usize;
        
        if file.len() < phoff + phnum * phentsize {
            return Err("Program headers fuera del archivo");
        }
        
        for i in 0..phnum {
            let phdr_ptr = (file.as_ptr() as usize + phoff + i * phentsize) as *const Elf64_Phdr;
            let phdr = unsafe { &*phdr_ptr };
            
            if phdr.p_type != PT_LOAD {
                crate::println!("  Segmento tipo {} ignorado", phdr.p_type);
                continue;
            }
            
            Self::load_segment(file, phdr)?;
        }
        
        Ok(())
    }
    
    fn load_segment(file: &[u8], phdr: &Elf64_Phdr) -> Result<(), &'static str> {
        let vaddr = phdr.p_vaddr as usize;
        let offset = phdr.p_offset as usize;
        let filesz = phdr.p_filesz as usize;
        let memsz = phdr.p_memsz as usize;
        
        crate::println!("  Cargando segmento LOAD en 0x{:x} ({} bytes)", vaddr, memsz);
        crate::println!("    flags: {}{}{}", 
            if phdr.p_flags & 1 != 0 { "X" } else { "-" },
            if phdr.p_flags & 2 != 0 { "W" } else { "-" },
            if phdr.p_flags & 4 != 0 { "R" } else { "-" });
        
        // Calcular páginas necesarias
        let page_count = (memsz + 4095) / 4096;
        let virt_start = VirtAddr::new(vaddr as u64);
        
        // Determinar flags de página
        let mut flags = PageTableFlags::PRESENT;
        if phdr.p_flags & 2 != 0 { flags |= PageTableFlags::WRITABLE; }
        
        // Mapear memoria
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
                crate::println!("    copiando {} bytes", filesz);
                core::ptr::copy_nonoverlapping(src.as_ptr(), dest, filesz);
            }
            
            // Limpiar BSS
            if memsz > filesz {
                let bss_start = dest.add(filesz);
                let bss_size = memsz - filesz;
                crate::println!("    limpiando BSS: {} bytes", bss_size);
                core::ptr::write_bytes(bss_start, 0, bss_size);
            }
        }
        
        Ok(())
    }
}

#![no_std]
#![no_main]

mod font;
mod framebuffer;
mod keyboard;
mod elf;
mod syscall;
mod memory;

use framebuffer::{Framebuffer, WRITER, INPUT_PROMPT};
use limine::request::{FramebufferRequest, MemoryMapRequest, HhdmRequest};
use limine::memory_map::EntryType;
use core::panic::PanicInfo;
use spin::Mutex;

static HELLO_ELF: &[u8] = include_bytes!("user/hello.elf");

#[used]
#[link_section = ".requests"]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: u64 = 0;

#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: u64 = 0;

// Offset HHDM global
pub static HHDM_OFFSET: Mutex<Option<u64>> = Mutex::new(None);

// Función para convertir física a virtual
pub fn phys_to_virt(phys: u64) -> u64 {
    if let Some(offset) = *HHDM_OFFSET.lock() {
        phys + offset
    } else {
        panic!("HHDM no inicializado");
    }
}

#[no_mangle]
extern "C" fn _start() -> ! {
    // Inicializar framebuffer
    let fb_response = FRAMEBUFFER_REQUEST.get_response()
        .expect("No se pudo obtener framebuffer");
    
    let fb = unsafe {
        Framebuffer::new_from_limine(fb_response)
            .expect("Error al crear framebuffer")
    };
    *WRITER.lock() = Some(fb);
    
    // Inicializar teclado
    keyboard::init();
    
    // Obtener HHDM offset
    let hhdm_response = HHDM_REQUEST.get_response().expect("No se pudo obtener HHDM");
    *HHDM_OFFSET.lock() = Some(hhdm_response.offset());
    
    // Obtener memory map
    let memory_map_response = MEMORY_MAP_REQUEST.get_response().expect("No se pudo obtener memory map");
    
    // Inicializar frame allocator
    memory::FRAME_ALLOCATOR.lock().init(memory_map_response);
    
    // Mostrar memory map (debug)
    println!("=== Memory Map ===");
    for entry in memory_map_response.entries() {
        let base = entry.base;
        let len = entry.length;
        let kind = match entry.entry_type {
            EntryType::USABLE => "Usable",
            EntryType::RESERVED => "Reserved",
            EntryType::ACPI_RECLAIMABLE => "ACPI Reclaim",
            EntryType::ACPI_NVS => "ACPI NVS",
            EntryType::BAD_MEMORY => "Bad Memory",
            EntryType::KERNEL_AND_MODULES => "Kernel",
            EntryType::BOOTLOADER_RECLAIMABLE => "Bootloader Reclaim",
            _ => "Unknown",
        };
        println!("  {:#018x} - {:#018x} : {} ({} KiB)", 
            base, base + len, kind, len / 1024);
    }
    println!("==================");
    
    println!("========================================");
    println!("   DUCKOS - Ejecutando programa ELF    ");
    println!("========================================");
    println!("");
    
    // Crear syscalls
    let mut syscalls = syscall::KernelSyscalls::new();
    
    // Cargar y ejecutar programa
    println!("Cargando programa hello.elf...");
    println!("Tamaño del ELF: {} bytes", HELLO_ELF.len());
    
    match elf::ElfLoader::load_and_execute(HELLO_ELF, &mut syscalls) {
        Ok(()) => {
            println!("✅ Programa ejecutado correctamente");
        }
        Err(e) => {
            println!("❌ Error al ejecutar programa: {}", e);
        }
    }
    
    println!("");
    println!("Volviendo al kernel...");
    println!("");
    print!("{}", INPUT_PROMPT);
    
    loop {
        keyboard::poll_keyboard();
        for _ in 0..100000 {
            core::hint::spin_loop();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("💥 KERNEL PANIC: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

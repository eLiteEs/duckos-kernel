#![no_std]
#![no_main]

mod font;
mod framebuffer;
mod keyboard;
mod elf;
mod syscall;

use framebuffer::{Framebuffer, WRITER, INPUT_PROMPT};
use limine::request::FramebufferRequest;
use core::panic::PanicInfo;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: u64 = 0;

#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: u64 = 0;

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
    
    // Mensaje de bienvenida
    println!("========================================");
    println!("   DUCKOS - Ahora con ELF y syscalls!  ");
    println!("========================================");
    println!("");
    
    // Cargar un programa ELF de ejemplo
    // Por ahora, simulamos un programa en memoria
    let fake_program = include_bytes!("../user/hello.elf");
    let mut syscalls = syscall::KernelSyscalls::new();
    
    match elf::ElfLoader::load_and_execute(fake_program, &mut syscalls) {
        Ok(()) => println!("Programa ejecutado correctamente"),
        Err(e) => println!("Error al ejecutar programa: {}", e),
    }
    
    // Bucle principal con polling de teclado
    println!("");
    print!("{}", INPUT_PROMPT);
    
    loop {
        keyboard::poll_keyboard();
        
        // Pequeña pausa
        for _ in 0..100000 {
            core::hint::spin_loop();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

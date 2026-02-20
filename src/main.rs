#![no_std]
#![no_main]

mod font;
mod framebuffer;
mod keyboard;

use framebuffer::{Framebuffer, WRITER, INPUT_PROMPT};
use limine::{
    request::FramebufferRequest,
    BaseRevision,
};
use core::panic::PanicInfo;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

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
    // 1. Obtener framebuffer
    let fb_response = match FRAMEBUFFER_REQUEST.get_response() {
        Some(response) => response,
        None => loop { x86_64::instructions::hlt(); }
    };

    // 2. Crear framebuffer
    let fb = match unsafe { Framebuffer::new_from_limine(fb_response) } {
        Some(fb) => fb,
        None => loop { x86_64::instructions::hlt(); }
    };

    // 3. Guardar en WRITER global
    *WRITER.lock() = Some(fb);

    // 4. Inicializar teclado
    keyboard::init();

    // 5. Mensaje de bienvenida
    println!("========================================");
    println!("   DUCKOS - Teclado por POLLING!       ");
    println!("========================================");
    println!("");
    println!("Presiona teclas (sin interrupciones):");
    println!("");
    print!("{}", INPUT_PROMPT);

    // 6. Bucle principal con polling del teclado
    loop {
        // Comprobar teclado continuamente
        keyboard::poll_keyboard();

        // Pequeña pausa para no saturar la CPU
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


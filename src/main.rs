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
    // Get framebuffer
    let fb_response = match FRAMEBUFFER_REQUEST.get_response() {
        Some(response) => response,
        None => loop { x86_64::instructions::hlt(); }
    };

    // Create frambuffer
    let fb = match unsafe { Framebuffer::new_from_limine(fb_response) } {
        Some(fb) => fb,
        None => loop { x86_64::instructions::hlt(); }
    };

    // Save in global WRITER
    *WRITER.lock() = Some(fb);

    // Init keyboard
    keyboard::init();

    println!("DuckOS v0.1.0");

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}


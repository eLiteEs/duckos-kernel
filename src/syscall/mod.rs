//! Módulo de llamadas al sistema

mod numbers;
mod handler;
mod context;

pub use numbers::*;
pub use handler::SyscallHandler;
pub use context::SyscallContext;

/// Trait que deben implementar los manejadores de syscalls
pub trait Syscalls {
    fn write(&mut self, fd: u32, buf: &[u8]) -> i32;
    fn read(&mut self, fd: u32, buf: &mut [u8]) -> i32;
    fn exit(&mut self, code: i32) -> !;
    // Añade más según necesites
}

/// Implementación por defecto para el kernel
pub struct KernelSyscalls {
    // Añade aquí lo que necesites (framebuffer, teclado, etc.)
}

impl KernelSyscalls {
    pub fn new() -> Self {
        Self {}
    }
}

impl Syscalls for KernelSyscalls {
    fn write(&mut self, fd: u32, buf: &[u8]) -> i32 {
        match fd {
            1 | 2 => { // stdout/stderr
                if let Ok(s) = core::str::from_utf8(buf) {
                    crate::print!("{}", s);
                }
                buf.len() as i32
            }
            _ => -1,
        }
    }
    
    fn read(&mut self, fd: u32, buf: &mut [u8]) -> i32 {
        match fd {
            0 => { // stdin
                // Aquí iría la lectura de teclado
                // Por ahora, devolvemos 0 (sin datos)
                0
            }
            _ => -1,
        }
    }
    
    fn exit(&mut self, code: i32) -> ! {
        crate::println!("Programa terminado con código: {}", code);
        loop {
            x86_64::instructions::hlt();
        }
    }
}

// src/syscall/mod.rs

pub trait Syscalls {
    fn write(&mut self, fd: u32, buf: &[u8]) -> i32;
    fn exit(&mut self, code: i32) -> !;
}

pub struct KernelSyscalls;

impl KernelSyscalls {
    pub fn new() -> Self {
        Self
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
            _ => {
                crate::println!("write a fd {} no implementado", fd);
                -1
            }
        }
    }
    
    fn exit(&mut self, code: i32) -> ! {
        crate::println!("Programa terminado con código: {}", code);
        // Por ahora, volvemos al kernel (esto debería ser un salto real)
        // Pero como es !, no podemos volver. Así que hacemos un loop.
        loop {
            x86_64::instructions::hlt();
        }
    }
}

//! Manejador de syscalls (para cuando implementes interrupciones)

use super::{Syscalls, SYS_READ, SYS_WRITE, SYS_EXIT};

pub struct SyscallHandler;

impl SyscallHandler {
    pub fn handle(
        syscall_num: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        syscalls: &mut dyn Syscalls,
    ) -> usize {
        match syscall_num {
            SYS_WRITE => {
                let fd = arg1 as u32;
                let buf_ptr = arg2 as *const u8;
                let count = arg3;
                
                let buf = unsafe { core::slice::from_raw_parts(buf_ptr, count) };
                syscalls.write(fd, buf) as usize
            }
            
            SYS_READ => {
                let fd = arg1 as u32;
                let buf_ptr = arg2 as *mut u8;
                let count = arg3;
                
                let buf = unsafe { core::slice::from_raw_parts_mut(buf_ptr, count) };
                syscalls.read(fd, buf) as usize
            }
            
            SYS_EXIT => {
                let code = arg1 as i32;
                syscalls.exit(code);
            }
            
            _ => {
                crate::println!("Syscall {} no implementada", syscall_num);
                !0 // -1 en usize
            }
        }
    }
}

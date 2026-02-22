#![no_std]
#![no_main]

// Usar números de syscall de Linux/x86_64
const SYS_WRITE: u64 = 1;
const SYS_EXIT: u64 = 60;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Escribir directamente en el framebuffer? No, mejor usar syscall
    let msg = b"HOLA\n";
    
    // Hacer syscall write(1, msg, 5)
    core::arch::asm!(
        "syscall",
        in("rax") SYS_WRITE,
        in("rdi") 1,
        in("rsi") msg.as_ptr(),
        in("rdx") msg.len(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );
    
    // Syscall exit(0)
    core::arch::asm!(
        "syscall",
        in("rax") SYS_EXIT,
        in("rdi") 0,
        options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") 1,
            options(noreturn)
        );
    }
}

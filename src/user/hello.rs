#![no_std]
#![no_main]

const SYS_WRITE: u64 = 1;
const SYS_EXIT: u64 = 60;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Mensaje 1
    let msg1 = b"INICIO: Programa ejecutandose!\n";
    core::arch::asm!(
        "syscall",
        in("rax") SYS_WRITE,
        in("rdi") 1,
        in("rsi") msg1.as_ptr(),
        in("rdx") msg1.len(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );
    
    // Mensaje 2 - EL QUE ESPERAS
    let msg2 = b"Hola desde DuckOS!\n";
    core::arch::asm!(
        "syscall",
        in("rax") SYS_WRITE,
        in("rdi") 1,
        in("rsi") msg2.as_ptr(),
        in("rdx") msg2.len(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );
    
    // Mensaje 3
    let msg3 = b"FIN: Saliendo...\n";
    core::arch::asm!(
        "syscall",
        in("rax") SYS_WRITE,
        in("rdi") 1,
        in("rsi") msg3.as_ptr(),
        in("rdx") msg3.len(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );
    
    // Salir
    core::arch::asm!(
        "syscall",
        in("rax") SYS_EXIT,
        in("rdi") 42,
        options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let msg = b"PANIC en programa!\n";
        core::arch::asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") 2,
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        );
        
        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") 1,
            options(noreturn)
        );
    }
}

#![no_std]
#![no_main]

const SYS_WRITE: u64 = 1;
const SYS_EXIT: u64 = 60;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let msg = b"Hola desde ELF64!\n";
    
    // Write syscall
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
    
    // Exit syscall
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

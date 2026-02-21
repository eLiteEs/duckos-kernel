//! Contexto de ejecución para syscalls (útil para guardar estado)

pub struct SyscallContext {
    pub rax: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub r10: u64,
    pub r8: u64,
    pub r9: u64,
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
}

impl SyscallContext {
    pub fn new() -> Self {
        Self {
            rax: 0, rdi: 0, rsi: 0, rdx: 0,
            r10: 0, r8: 0, r9: 0,
            rip: 0, rsp: 0, rflags: 0,
        }
    }
}

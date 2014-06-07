#[no_mangle]
#[no_split_stack]
pub unsafe extern fn _start() {
    asm!("pop   %rdi");
    asm!("mov   %rsp,%rsi");
    asm!("push  %rdi");
    asm!("call  main");
    asm!("mov   %rax,%rdi");
    asm!("mov   $$60,%rax");
    asm!("syscall");
}

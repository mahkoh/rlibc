#[no_mangle]
pub unsafe extern fn _start() {
    asm!("pop   %rdi
    	mov   %rsp,%rsi
    	push  %rdi
    	call  main
    	mov   %rax,%rdi
    	mov   $$60,%rax
    	syscall");
}

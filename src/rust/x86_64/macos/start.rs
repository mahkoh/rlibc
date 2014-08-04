#[no_mangle]
#[no_split_stack]
pub unsafe extern fn start() {
    asm!("	pop   %rdi
    		mov   %rsp,%rsi
    		push  %rdi
    		call  _main"
    	:::: "volatile");

    asm!("	mov   %rax,%rdi
    		mov   $$0x2000001,%rax
    		syscall"
    	:::: "volatile");
}

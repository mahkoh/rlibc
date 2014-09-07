#[no_mangle]
#[no_split_stack]
pub unsafe extern fn start() {
	// THIS IS AUTO-INSERTED BY COMPILER:
	// pushq	%rbp
	// movq	%rsp, %rbp

	// Pop nonsense %rbp value.
	// Mark deepest frame with 0.
	asm!("	pop   %rdi
			push  $$0" :::: "volatile");

	// Load argc and argv as 1st & 2nd args.
	asm!("	mov   +8(%rsp),%rdi
    		lea   +16(%rsp),%rsi
    		call  _crt0" :::: "volatile");

	// Load argc and argv as 1st & 2nd args.
	asm!("	mov   +8(%rsp),%rdi
    		lea   +16(%rsp),%rsi
    		call  _main" :::: "volatile");

    asm!("	mov   %rax,%rdi
    		mov   $$0x2000001,%rax
    		syscall" :::: "volatile");
}

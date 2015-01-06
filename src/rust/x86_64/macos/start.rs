use rust::prelude::*;
use types::{int_t, char_t};
use posix::stdlib::{ARGV, ARGC, ENVP, ENVC, APPLE, exit};

extern "C" {
	fn main(argc: int_t,
			argv: *const *const char_t,
			envp: *const *const char_t,
			apple: *const *const char_t) -> int_t;
}

/// This function is still mangled to "_start", yet the linker looks for
/// "start". Also, Rust inserts the frame-pointer prelude, which is invalid
/// for an executable's entry point.
#[no_mangle]
pub unsafe extern fn start() {
	// THIS IS AUTO-INSERTED BY COMPILER:
	// pushq	%rbp
	// movq	%rsp, %rbp

	// Pop nonsense %rbp value.
	// Mark deepest frame with 0.
	asm!("	pop   %rdi
		start:
			push  $$0
			movq  %rsp, %rbp
			mov   +8(%rsp), $0
			lea   +16(%rsp), $1"
			: "=r"(ARGC), "=r"(ARGV) ::: "volatile");

	ENVP = offset(ARGV, ARGC as int + 1);

	let mut apple: *const *const char_t = ENVP;
    while (*apple as uint != 0) {
        apple = offset(apple, 1); // increases by one pointer size
    }
    ENVC = (apple as uint - ENVP as uint - 1);
    apple = offset(apple, 1); // one NULL pointer separates apple[] from env[]
    APPLE = apple;

	let status = main(ARGC as int_t, ARGV, ENVP, apple);

	exit(status);
}

use rust::prelude::*;
use types::{int_t, char_t};
use posix::stdlib::{ARGV, ARGC, ENVP, ENVC, exit};

extern "C" {
	fn main(argc: int_t,
			argv: *const *const char_t,
			envp: *const *const char_t) -> int_t;
}

/// FIXME Rust issue #20659
#[inline(never)]
unsafe fn call_main(argc: int_t,
                    argv: *const *const char_t,
                    envp: *const *const char_t,) {
    asm!("  call main
            mov   %rax,%rdi
            mov   $$60,%rax
            syscall
            " :: "{rdi}"(argc), "{rsi}"(argv), "{rdx}"(envp));
}

/// This function is called _start to match the OS X target's name mangling.
/// It stores the addresses of the stack arguments, invokes main(), and passes
/// the return status to exit().
#[no_mangle]
pub unsafe fn _start() {
    asm!("	mov (%rsp), $0
    		lea +8(%rsp), $1"
    		: "=r"(ARGC), "=r"(ARGV) ::: "volatile");

    ENVP = offset(ARGV, ARGC as int + 1);

    let mut envc: *const *const char_t = ENVP;
    while (*envc as uint != 0) {
        envc = offset(envc, 1); // increases by one pointer size
    }
    ENVC = (envc as uint - ENVP as uint - 1);

    call_main(ARGC as int_t, ARGV, ENVP);
}

use rust::prelude::*;
use types::{int_t, char_t};
use posix::stdlib::{ARGV, ARGC, ENVP, ENVC, exit};

extern "C" {
	fn main(argc: int_t,
			argv: *const *const char_t,
			envp: *const *const char_t) -> int_t;
}

/// This function is called by start().
/// It stores the addresses of the stack arguments, invokes main(), and passes
/// the return status to exit().
#[no_mangle]
pub unsafe extern fn __libc_start_main(argc: uint, argv: *const *const char_t) {
    ARGC = argc;
    ARGV = argv;
    ENVP = offset(ARGV, ARGC as int + 1);

    let mut envc: *const *const char_t = ENVP;
    while (*envc as uint != 0) {
        envc = offset(envc, 1); // increases by one pointer size
    }
    ENVC = (envc as uint - ENVP as uint - 1);

    call_main(ARGC as int_t, ARGV, ENVP);
}

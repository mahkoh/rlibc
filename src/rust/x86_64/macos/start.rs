use rust::prelude::*;
use types::{int_t, char_t};
use posix::stdlib::{ARGV, ARGC, ENVP, ENVC, APPLE, exit};

extern "C" {
	fn main(argc: int_t,
			argv: *const *const char_t,
			envp: *const *const char_t,
			apple: *const *const char_t) -> int_t;
}

/// This function is mangled to "_libc_start_main", which the linker looks.
/// Also, Rust inserts the frame-pointer prelude, which is invalid
/// for an executable's entry point.
#[no_mangle]
pub unsafe extern fn _libc_start_main(argc: uint, argv: *const *const char_t) {
	ARGC = argc;
    ARGV = argv;
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

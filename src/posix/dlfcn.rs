//! Dynamic linking or loading is evil and thus not supported.

use types::{char_t, int_t, void_t};
use posix::stdlib::{exit};

#[no_mangle]
pub fn dlopen(filename: *const char_t, flag: int_t) -> *const void_t {
	exit(1);
}

#[no_mangle]
pub fn dlerror() -> *const char_t {
	exit(1);
}

#[no_mangle]
pub fn dlsym(handle: *const void_t, symbol: *const char_t) -> *const void_t {
	exit(1);
}

#[no_mangle]
pub fn dlclose(handle: *const void_t) -> int_t {
	exit(1);
}

#[no_mangle]
pub fn dladdr(addr: *const void_t, info: *const void_t) -> int_t {
	exit(1);
}

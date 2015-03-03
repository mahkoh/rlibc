//! Dynamic linking or loading is evil and thus not supported.

use types::{char_t, int_t, void_t};
use posix::pm::{exit};

#[no_mangle]
pub unsafe fn dlopen(_filename: *const char_t, _flag: int_t) -> *const void_t {
	exit(1);
}

#[no_mangle]
pub unsafe fn dlerror() -> *const char_t {
	exit(1);
}

#[no_mangle]
pub unsafe fn dlsym(_handle: *const void_t, _symbol: *const char_t) -> *const void_t {
	exit(1);
}

#[no_mangle]
pub unsafe fn dlclose(_handle: *const void_t) -> int_t {
	exit(1);
}

#[no_mangle]
pub unsafe fn dladdr(_addr: *const void_t, _info: *const void_t) -> int_t {
	exit(1);
}

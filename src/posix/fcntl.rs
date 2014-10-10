use types::{char_t, int_t, mode_t};
use syscalls::{sys_open};
use libc::errno::{errno};

#[no_mangle]
pub unsafe extern fn open(path: *const char_t, flags: int_t, mode: mode_t) -> int_t {
    /*
    match sys_open(path, flags, mode) {
        n if n < 0 => {
            errno = -n;
            -1
        },
        n => n,
    }
    */
    0
}

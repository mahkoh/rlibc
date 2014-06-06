use types::{char_t, int_t};
use syscalls::{sys_unlink, sys_rmdir};
use libc::errno::{errno};

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn unlink(file: *char_t) -> int_t {
    match sys_unlink(file) {
        n if n < 0 => {
            errno = -n;
            -1
        },
        _ => 0,
    }
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn rmdir(file: *char_t) -> int_t {
    match sys_rmdir(file) {
        n if n < 0 => {
            errno = -n;
            -1
        },
        _ => 0,
    }
}

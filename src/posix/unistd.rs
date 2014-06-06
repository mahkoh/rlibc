use types::{char_t, int_t};
use syscalls::{sys_unlink};
use libc::errno::{errno};

#[no_mangle]
#[no_split_stack]
pub extern fn unlink(file: *char_t) -> int_t {
    match unsafe { sys_unlink(file) } {
        n if n < 0 => {
            unsafe { errno = -n; }
            -1
        },
        _ => 0,
    }
}

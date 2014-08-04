use types::{char_t, int_t, void_t, size_t, ssize_t, uint_t};
use syscalls::{sys_unlink, sys_rmdir, sys_read, sys_close};
use libc::errno::{errno};

macro_rules! forward {
    ($sys:ident, $($p:expr),*) => {
        match $sys($($p),+) {
            n if n < 0 => {
                errno = -n;
                -1
            },
            n => n,
        }
    };
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn unlink(file: *const char_t) -> int_t {
    forward!(sys_unlink, file)
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn rmdir(file: *const char_t) -> int_t {
    forward!(sys_rmdir, file)
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn close(fd: int_t) -> int_t {
    forward!(sys_close, fd as uint_t)
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn read(fd: int_t, buf: *mut void_t, count: size_t) -> ssize_t {
    (forward!(sys_read, fd as uint_t, buf as *mut char_t, count)) as ssize_t
}

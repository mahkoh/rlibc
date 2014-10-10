use types::{char_t, int_t, void_t, size_t, ssize_t, off_t, uint_t, ulong_t};
use syscalls::{sys_unlink, sys_rmdir, sys_read, sys_close, sys_lseek};

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use syscalls::{sys_pread64, sys_pwrite64};

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use syscalls::{sys_pread, sys_pwrite};

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
pub unsafe extern fn unlink(file: *const char_t) -> int_t {
    forward!(sys_unlink, file)
}

#[no_mangle]
pub unsafe extern fn rmdir(file: *const char_t) -> int_t {
    forward!(sys_rmdir, file)
}

#[no_mangle]
pub unsafe extern fn close(fd: int_t) -> int_t {
    forward!(sys_close, fd as uint_t)
}

#[no_mangle]
pub unsafe extern fn read(fd: int_t, buf: *mut void_t, count: size_t) -> ssize_t {
    (forward!(sys_read, fd as uint_t, buf as *mut char_t, count)) as ssize_t
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern fn pread(fd: int_t, buf: *mut void_t, count: size_t, offset: off_t)
    -> ssize_t {
    (forward!(sys_pread64, fd as ulong_t, buf as *mut char_t, count, offset) as ssize_t)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern fn pread(fd: int_t, buf: *mut void_t, count: size_t, offset: off_t)
    -> ssize_t {
    (forward!(sys_pread, fd, buf as *mut char_t, count, offset) as ssize_t)
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern fn pwrite(fd: int_t, buf: *const void_t, count: size_t, offset: off_t)
    -> ssize_t {
    (forward!(sys_pwrite64, fd as uint_t, buf as *const char_t, count, offset) as ssize_t)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern fn pwrite(fd: int_t, buf: *const void_t, count: size_t, offset: off_t)
    -> ssize_t {
    (forward!(sys_pwrite, fd, buf as *const char_t, count, offset) as ssize_t)
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern fn lseek(fd: int_t, offset: off_t, whence: int_t) -> off_t {
    (forward!(sys_lseek, fd as uint_t, offset, whence as uint_t) as off_t)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern fn lseek(fd: int_t, offset: off_t, whence: int_t) -> off_t {
    (forward!(sys_lseek, fd, offset, whence) as off_t)
}

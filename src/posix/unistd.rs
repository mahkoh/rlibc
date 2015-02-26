use types::{char_t, int_t, void_t, size_t, ssize_t, uint_t};
use types::{off_t};
use types::{pid_t, uid_t, gid_t};
use syscalls::{sys_unlink, sys_rmdir, sys_read, sys_write, sys_close, sys_lseek};
use syscalls::{sys_getpid, sys_getuid, sys_geteuid, sys_setuid, sys_setgid, sys_setsid};

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use syscalls::{sys_pread64, sys_pwrite64};

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use syscalls::{sys_pread, sys_pwrite};

use libc::errno::{errno};

macro_rules! forward {
    ($sys:ident, $($p:expr),*) => {
        match $sys($($p),*) {
            n if n < 0 => {
                errno = -n;
                -1
            },
            n => n,
        }
    };
}

// File and filesystem manipulation

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
pub unsafe extern fn write(fd: int_t, buf: *const void_t, count: size_t) -> ssize_t {
    (forward!(sys_write, fd as uint_t, buf as *const char_t, count)) as ssize_t
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


// Environment

#[no_mangle]
pub unsafe extern fn getpid() -> pid_t {
    (forward!(sys_getpid,) as pid_t)
}

#[no_mangle]
pub unsafe extern fn getuid() -> uid_t {
    (forward!(sys_getuid,) as uid_t)
}

#[no_mangle]
pub unsafe extern fn geteuid() -> uid_t {
    (forward!(sys_geteuid,) as uid_t)
}

#[no_mangle]
pub unsafe extern fn setuid(uid: uid_t) -> int_t {
    forward!(sys_setuid, uid)
}

#[no_mangle]
pub unsafe extern fn setgid(gid: gid_t) -> int_t {
    forward!(sys_setgid, gid)
}

#[no_mangle]
pub unsafe extern fn setsid() -> pid_t {
    (forward!(sys_setsid,) as pid_t)
}

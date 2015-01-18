//! Memory management

use types::{int_t, uint_t, ulong_t, void_t, size_t, intptr_t};
use types::{off_t};
use types::{rlimit};
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use types::{caddr_t};

use syscalls::{sys_getrlimit, sys_mmap, sys_munmap};
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use syscalls::{sys_brk};

use rust::prelude::*;

use consts::errno::{ENOMEM};
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

/// Increases the data break to the given address, returning 0 on success
/// or -1 on failure, setting errno to ENOMEM.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern fn brk(addr: *const void_t) -> int_t {
    let oldbrk = sys_brk(0) as usize;
    match sys_brk(addr as ulong_t) as usize != oldbrk {
        true => 0,
        false => {
            errno = ENOMEM;
            -1
        }
    }
}

/// Increments the data break by `increment`, returning either the previous
/// break or `((void*)-1)` on failure, setting errno to ENOMEM.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern fn sbrk(increment: intptr_t) -> *const void_t {
    let oldbrk = sys_brk(0) as *const u8;
    if increment != 0 {
        let newbrk = offset(oldbrk, increment as isize);
        if sys_brk(newbrk as ulong_t) as *const u8 != newbrk {
            errno = ENOMEM;
            -1 as *const void_t
        } else {
            oldbrk
        }
    } else {
        oldbrk
    }
}

/// Get resource limits. For RLIMIT_DATA, this is the maximum size of the
/// process's data segment. This limit affects calls to brk(2) and sbrk(2).
#[no_mangle]
pub unsafe extern fn getrlimit(resource: int_t, rlim: *mut rlimit) -> int_t {
    (forward!(sys_getrlimit, resource as uint_t, rlim))
}

/// Map or unmap files or devices into memory.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern fn mmap(
        addr: *const void_t, length: size_t, prot: int_t, flags: int_t,
        fd: int_t, offset: off_t) -> *const void_t {
    forward!(sys_mmap, addr as ulong_t, length as ulong_t, prot as ulong_t,
        flags as ulong_t, fd as ulong_t, offset as ulong_t) as *const void_t
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern fn mmap(
        addr: *const void_t, length: size_t, prot: int_t, flags: int_t,
        fd: int_t, offset: off_t) -> *const void_t {
    forward!(sys_mmap, addr as caddr_t, length, prot, flags, fd, offset)
        as *const void_t
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern fn munmap(addr: *const void_t, length: size_t) -> int_t {
    forward!(sys_munmap, addr as ulong_t, length)
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern fn munmap(addr: *const void_t, length: size_t) -> int_t {
    forward!(sys_munmap, addr as caddr_t, length)
}

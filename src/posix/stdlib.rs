use rust::prelude::*;
use types::{char_t, int_t, size_t};
use libc::string::{strlen, strncpy, strnlen, strncmp};
use libc::errno::{errno};
use consts::NULL;
use consts::errno::{EINVAL, EEXIST};
use posix::fcntl::{open};
use posix::signal::{raise, SIGABRT};

use syscalls::{sys_exit};

pub static mut ARGV: *const *const char_t = 0u as *const *const char_t;
pub static mut ARGC: uint = 0;
pub static mut ENVP: *const *const char_t = 0u as *const *const char_t;
pub static mut ENVC: uint = 0;

#[cfg(target_os = "macos")]
pub static mut APPLE: *const *const char_t = 0u as *const *const char_t;

const K_ENV_MAXKEYLEN: size_t = 512;

#[no_mangle]
pub unsafe extern fn get_argv() -> &'static [*const char_t] {
    transmute(from_raw_buf(
        &ARGV,
        ARGC
    ))
}

#[no_mangle]
pub unsafe extern fn get_envp() -> &'static [*const char_t] {
    transmute(from_raw_buf(
        &ENVP,
        ENVC
    ))
}

#[no_mangle]
#[cfg(target_os = "macos")]
pub unsafe extern fn _NSGetArgc() -> *const int_t {
    (&ARGC) as *const uint as *const int_t
}

#[no_mangle]
#[cfg(target_os = "macos")]
pub unsafe extern fn _NSGetArgv() -> *const *const *const char_t {
    (&ARGV) as *const *const *const char_t
}

#[no_mangle]
#[cfg(target_os = "macos")]
pub unsafe extern fn _NSGetEnviron() -> *const *const *const char_t {
    (&ENVP) as *const *const *const char_t
}

#[no_mangle]
#[cfg(target_os = "macos")]
pub unsafe extern fn _NSGetProgname() -> *const *const char_t {
    APPLE // apple[0] should point to the binary's path
}

#[no_mangle]
#[cfg(target_os = "macos")]
pub unsafe extern fn _NSGetExecutablePath(buf: *mut char_t, size: *mut u32) -> int_t {
    let len = strlen(*APPLE);
    if len < *size as size_t {
        strncpy(buf, *APPLE, len);
        0
    } else {
        *size = len as u32;
        -1
    }
}

#[no_mangle]
pub unsafe extern fn getenv(key: *const char_t) -> *const char_t {
    let len = strnlen(key, K_ENV_MAXKEYLEN);
    for &env in get_envp().iter() {
        if strncmp(key, env, len) == 0 && *offset(env, len as int) == '=' as i8 {
            return offset(env, (len as int) + 1)
        }
    }
    0 as *const char_t
}

#[no_mangle]
pub unsafe extern fn setenv(key: *const char_t,
                            val: *const char_t,
                            overwrite: int_t) -> int_t {
    _exit(1); // TODO implement mutable environment
}

#[no_mangle]
pub unsafe extern fn unsetenv(key: *const char_t) -> int_t {
    _exit(1); // TODO implement mutable environment
}

/// Terminates the process normally, performing the regular cleanup.
/// All C streams are closed, and all files created with tmpfile are removed.
/// Status can be zero or EXIT_SUCCESS, or EXIT_FAILURE.
#[no_mangle]
pub extern fn exit(x: int_t) -> ! {
    _exit(x);
}

/// _Exit is a synonym for _exit
#[no_mangle]
pub extern fn _Exit(x: int_t) -> ! {
    _exit(x);
}

#[no_mangle]
pub extern fn _exit(x: int_t) -> ! {
    unsafe {sys_exit(x);}
    loop { }; // for divergence check
}

#[no_mangle]
pub unsafe extern fn abort() {
    raise(SIGABRT);
}

/*
#[no_mangle]
pub unsafe extern fn mkstemp(tplt: *mut char_t) -> int_t {
    let slc = tplt.to_mut_slice(strlen(tplt as *_) as uint);
    if slc.len() < 6 || slc.lastn(6).iter().any(|c| *c != cc!('X')) {
        errno = EINVAL;
        return -1;
    }
    let rand = os_rand();
    let mut buf: [char_t, ..6] = uninit();
    loop {
        rand.fill(buf);
        for (i, c) in slc.lastn(6).mut_iter().enumerate() {
            match buf[i] & 15 {
                0..9   => *c = buf[i] + cc!('0'),
                10..15 => *c = buf[i] + cc!('a') - 10,
            }
        }
        match open(tplt, ) {
            i if i >= 0          => return i,
            _ if errno != EEXIST => return -1,
            _ => { }
        }
    }
}
*/

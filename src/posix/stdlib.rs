use rust::prelude::*;
use types::{char_t, int_t};
use libc::string::{strlen};
use libc::errno::{errno};
use consts::NULL;
use consts::errno::{EINVAL, EEXIST};
use posix::fcntl::{open};

use core::slice::raw::buf_as_slice;
use core::mem::transmute;
use syscalls::{sys_exit};

pub static mut ARGV: &'static [*const u8] = &[];
pub static mut ENVP: &'static [*const u8] = &[];


#[no_mangle]
pub unsafe extern fn crt0(argc: int, argv: *const *const u8) {
    ARGV = buf_as_slice(
        argv,
        argc as uint,
        |r: &[*const u8]| -> &'static [*const u8] {transmute(r)}
    );

    let envp: *const *const u8 = offset(argv, argc+1);
    let mut envc = 0i;
    while (*offset(envp, envc) as uint != 0u) {
        envc += 1;
    }

    ENVP = buf_as_slice(
        envp,
        envc as uint,
        |r: &[*const u8]| -> &'static [*const u8] {transmute(r)}
    );
}

#[no_mangle]
pub unsafe extern fn getarg(index: int_t) -> *const char_t {
    match ARGV.get(index as uint) {
        Some(arg) => *arg as *const char_t,
        None => NULL as *const char_t
    }
}

#[no_mangle]
pub unsafe extern fn getenv(index: int_t) -> *const char_t {
    match ENVP.get(index as uint) {
        Some(env) => *env as *const char_t,
        None => NULL as *const char_t
    }
}

#[no_mangle]
pub fn exit(x: int_t) -> ! {
    _Exit(x);
}

#[no_mangle]
pub fn _Exit(x: int_t) -> ! {
    unsafe {sys_exit(1);}
    loop { }; // for divergence check
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

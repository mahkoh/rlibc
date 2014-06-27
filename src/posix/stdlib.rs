use rust::prelude::*;
use types::{char_t, int_t};
use libc::string::{strlen};
use libc::errno::{errno};
use consts::errno::{EINVAL, EEXIST};
use posix::fcntl::{open};

/*
#[no_mangle]
#[no_split_stack]
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

use consts::NULL;
use types::*;
use syscalls::sys_gettimeofday;

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
pub unsafe fn time(time: *mut time_t) -> time_t {
	let mut now: timeval = timeval {tv_sec: 0xabcd, tv_usec: 0xabcd};
	if forward!(sys_gettimeofday,
				&mut now as *mut timeval,
				NULL as *mut timezone) >= 0 {
		if time != NULL as *mut time_t {
			*time = now.tv_sec;
		}
		now.tv_sec
	} else {
		-1
	}
}

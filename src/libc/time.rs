use rust::prelude::*;
use consts::NULL;
use types::{int_t, char_t, long_t, time_t, tm, timeval, timezone};
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
pub unsafe extern fn time(time: *mut time_t) -> time_t {
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

#[no_mangle]
pub unsafe extern fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> int_t {
	forward!(sys_gettimeofday, tv, tz)
}

pub static mut gmtime_tm: tm = tm {
	tm_sec: 0,
	tm_min: 0,
	tm_hour: 0,
	tm_mday: 0,
	tm_mon: 0,
	tm_year: 0,
	tm_wday: 0,
	tm_yday: 0,
	tm_isdst: 0,
	tm_gmtoff: 0,
	tm_zone: 0 as *mut char_t,
};

static TM_ZONE_GMT: [i8; 4] = ['G' as i8, 'M' as i8, 'T' as i8, 0];

const EPOCH_YR: int_t = 1970;
const YR_1900: int_t = 1900;
const SECS_DAY: u64 = 86400;

/// Length of the passed gregorian year (e.g. 1970).
fn YEARSIZE(year: int_t) -> u64 {
	match year {
		// leap years on non-centennials
		y if (y % 4 == 0 && y % 100 != 0) => 366,
		// leap years on centennials not multiples of 400
		y if (y % 4 == 0 && y % 100 == 0 && y % 400 != 0) => 365,
		// leap years on multiples of 400
		y if (y % 4 == 0 && y % 100 == 0 && y % 400 == 0) => 366,
		// normal non-leap years. This doesn't exhaust for some reason:
		// y if (y % 4 != 0) => 365,
		_ => 365
	}
}

fn LEAPYEAR(year: int_t) -> bool {
	YEARSIZE(year) == 366
}

fn MONTHLEN(ly: bool, mon: int_t) -> u64 {
	let _ytab = &[
		[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
		[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
	];
	_ytab[ly as uint][mon as uint]
}

/// TODO negative times
#[no_mangle]
pub unsafe extern fn gmtime(timer: *const time_t) -> *const tm {
	let time: time_t = *timer;
	let dayclock: u64 = time as u64 % SECS_DAY;
	let mut dayno: u64 = time as u64 / SECS_DAY;
	let mut year: int_t = EPOCH_YR;

	gmtime_tm = tm {
		tm_sec: (dayclock % 60) as int_t,
		tm_min: ((dayclock % 3600) / 60) as int_t,
		tm_hour: (dayclock / 3600) as int_t,
		tm_wday: ((dayno + 4) % 7) as int_t, // day 0 was a thursday

		tm_year: {
			while dayno >= YEARSIZE(year) {
				dayno -= YEARSIZE(year);
				year += 1;
			}
			year - YR_1900
		},

		tm_yday: dayno as int_t,
		tm_mon: {
			let mut mon = 0;
			while dayno >= MONTHLEN(LEAPYEAR(year), mon) {
				dayno -= MONTHLEN(LEAPYEAR(year), mon);
				mon += 1;
			}
			mon
		},

		tm_mday: dayno as int_t + 1,
		tm_isdst: 0,
		tm_gmtoff: 0,
		tm_zone: TM_ZONE_GMT.as_ptr() as *mut char_t,
	};

	&gmtime_tm as *const tm
}

/// TODO negative times
/// TODO thread-local storage
#[no_mangle]
pub unsafe extern fn gmtime_r(timer: *const time_t) -> *const tm {
	gmtime(timer)
}

/// TODO time localization
#[no_mangle]
pub unsafe extern fn localtime(timer: *const time_t) -> *const tm {
	gmtime(timer)
}

/// TODO time localization
/// TODO thread-local storage
#[no_mangle]
pub unsafe extern fn localtime_r(timer: *const time_t) -> *const tm {
	gmtime_r(timer)
}

/// Convert a GMT `struct tm` to a time_t.
#[no_mangle]
pub unsafe extern fn timegm(timer_ptr: *const tm) -> time_t {
	let timer: &tm = transmute(timer_ptr);
	let yr = (timer.tm_year + EPOCH_YR);

	let mut t = (yr as time_t -1970) * (YEARSIZE(yr) * SECS_DAY) as time_t;
	t += timer.tm_yday as time_t * SECS_DAY as time_t;
	t += (timer.tm_hour * 3600 + timer.tm_min * 60 + timer.tm_sec) as time_t;
	t
}

/// TODO time localization
#[no_mangle]
pub unsafe extern fn mktime(timer_ptr: *const tm) -> time_t {
	timegm(timer_ptr)
}

#[no_mangle]
static mut tzname: [*mut char_t; 2] = [0 as *mut char_t, 0 as *mut char_t];
static mut timezone: long_t = 0;
static mut daylight: int_t = 0;

/// TODO time localization
#[no_mangle]
pub unsafe extern fn tzset() {
	tzname[0] = TM_ZONE_GMT.as_ptr() as *mut char_t;
	tzname[1] = TM_ZONE_GMT.as_ptr() as *mut char_t;
}

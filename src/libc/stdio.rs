use rust::prelude::*;

use types::{int_t, char_t, size_t};

use consts::errno::{EISDIR, EEXIST};

use libc::errno::{errno};
use libc::string::{strlen};

use posix::unistd::{unlink, rmdir};

use syscalls::{sys_rename, sys_write};

static _IOFBF: int_t = 0;
static _IOLBF: int_t = 1;
static _IONBF: int_t = 2;

static BUFSIZ: int_t = 8192;

static EOF: int_t = -1;

static FOPEN_MAX: int_t = 16;
static FILENAME_MAX: int_t = 4096;
static L_tmpnam: int_t = 20;

static SEEK_SET: int_t = 0;
static SEEK_CUR: int_t = 1;
static SEEK_END: int_t = 2;

static TMP_MAX: int_t = 238328;

struct FILE {
    fd: int_t,
}

#[no_mangle]
pub static mut __stdin: FILE = FILE { fd: 0 };
#[no_mangle]
pub static mut __stdout: FILE = FILE { fd: 1 };
#[no_mangle]
pub static mut __stderr: FILE = FILE { fd: 2 };

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn remove(file: *const char_t) -> int_t {
    if unlink(file) == -1 {
        match errno {
            EISDIR => rmdir(file),
            _      => -1,
        }
    } else {
        0
    }
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn rename(old: *const char_t, new: *const char_t) -> int_t {
    match sys_rename(old, new) {
        n if n < 0 => {
            errno = -n;
            -1
        },
        _ => 0,
    }
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn puts(s: *const char_t) -> int_t {
    let len = strlen(s);
    if sys_write(1, s, len) as size_t != len || sys_write(1, cs!("\n"), 1) != 1 {
        -1
    } else {
        0
    }
}

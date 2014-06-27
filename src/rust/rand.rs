use types::{int_t, size_t, char_t};

use consts::fcntl::{O_RDONLY};

use rust::{size_of, Repr};
use rust::lang::*;

use posix::unistd::{close, read};
use posix::fcntl::{open};

pub struct FD {
    fd: int_t,
}

impl FD {
    pub fn raw(&self) -> int_t {
        self.fd
    }
}

impl Drop for FD {
    #[no_split_stack]
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}

pub trait Rand {
    fn fill<T>(&self, &mut [T]);
}

pub struct OSRand {
    fd: FD,
}

impl Rand for OSRand {
    #[no_split_stack]
    fn fill<T>(&self, dst: &mut [T]) {
        let raw = dst.repr();
        unsafe {
            read(self.fd.raw(), raw.data as *mut _, (raw.len * size_of::<T>()) as size_t);
        }
    }
}

pub fn os_rand() -> OSRand {
    let fd = unsafe { open(cs!("/dev/urandom"), O_RDONLY, 0) };
    OSRand { fd: FD { fd: fd } }
}

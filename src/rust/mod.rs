#![macro_escape]

use syscalls::{sys_exit};

pub mod macros;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod slice;
pub mod iter;
pub mod option;
pub mod lang;
pub mod clone;
pub mod num;
pub mod rand;

pub mod prelude {
    pub use super::slice::{ImmutableSlice, Items, ToSlice, ToMutSlice};
    pub use super::iter::{Iterator, Zip, range, loop_from, DoubleEndedIterator};
    pub use super::option::{Option, Some, None};
    pub use super::{Repr, offset, offset_mut, uninit};
    pub use super::lang::*;
    pub use super::clone::{Clone};
    pub use super::num::{One};
    pub use super::rand::{Rand, os_rand};
}

extern "rust-intrinsic" {
    pub fn size_of<T>() -> uint;
    pub fn offset<T>(dst: *T, n: int) -> *T;
    pub fn transmute<T,U>(e: T) -> U;
    pub fn uninit<T>() -> T;
    pub fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *T, count: uint);
}

#[inline(always)]
#[no_split_stack]
pub unsafe fn offset_mut<T>(dst: *mut T, n: int) -> *mut T {
    offset(dst as *T, n) as *mut T
}

#[no_split_stack]
pub unsafe fn transmute_copy<T,U>(p: &T) -> U {
    let p = p as *T as *U;
    let mut u = uninit();
    copy_nonoverlapping_memory(&mut u, p, 1);
    u
}

pub trait Repr<T> {
    #[inline]
    #[no_split_stack]
    fn repr(&self) -> T {
        unsafe { transmute_copy(self) }
    }
}

#[cold]
#[lang = "fail_bounds_check"]
#[no_split_stack]
#[inline(never)]
fn fail_bounds_check(_: &'static str, _: uint, _: uint, _: uint) -> ! {
    unsafe { sys_exit(134); }
    loop { }
}

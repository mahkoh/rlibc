#![macro_escape]

use syscalls::{sys_exit};

pub mod macros;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod rand;

pub mod prelude {
    pub use core::slice::{ImmutableVector, Items, Vector, MutableVector};
    pub use core::iter::{Iterator, Zip, range, count, DoubleEndedIterator};
    pub use core::option::{Option, Some, None};
    pub use core::raw::{Repr};
    pub use core::intrinsics::{offset, uninit, copy_nonoverlapping_memory};
    pub use core::ops::*;
    pub use core::clone::{Clone};
    pub use core::num::{One};
    pub use super::rand::{Rand, os_rand};

    #[inline(always)]
    #[no_split_stack]
    pub unsafe fn offset_mut<T>(dst: *mut T, n: int) -> *mut T {
        offset(dst as *const T, n) as *mut T
    }

    #[no_split_stack]
    pub unsafe fn transmute_copy<T,U>(p: &T) -> U {
        let p = p as *const T as *const U;
        let mut u = uninit();
        copy_nonoverlapping_memory(&mut u, p, 1);
        u
    }
}

// extern "rust-intrinsic" {
//     pub fn size_of<T>() -> uint;
//     pub fn offset<T>(dst: *const T, n: int) -> *const T;
//     pub fn transmute<T,U>(e: T) -> U;
//     pub fn uninit<T>() -> T;
//     pub fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T, count: uint);
// }


// pub trait Repr<T> {
//     #[inline]
//     #[no_split_stack]
//     fn repr(&self) -> T {
//         unsafe { transmute_copy(self) }
//     }
// }

// #[cold]
// #[lang = "fail_bounds_check"]
// #[no_split_stack]
// #[inline(never)]
// fn fail_bounds_check(_: &'static str, _: uint, _: uint, _: uint) -> ! {
//     unsafe { sys_exit(134); }
//     loop { }
// }

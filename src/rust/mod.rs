//! The Rust core prelude.

#[macro_use]
pub mod macros;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod rand;

pub mod prelude {
    pub use core::slice::{SliceExt, from_raw_buf};
    pub use core::iter::{Iterator, IteratorExt, Zip, range, count, DoubleEndedIterator};
    pub use core::option::Option::{self, Some, None};
    pub use core::raw::{Repr};
    pub use core::intrinsics::{offset, uninit, copy_nonoverlapping_memory};
    pub use core::ops::*;
    pub use core::ptr::{PtrExt};
    pub use core::clone::{Clone};
    pub use core::num::{Int, Float};
    pub use core::mem::transmute;
    pub use super::rand::{Rand, os_rand};

    #[inline(always)]
    pub unsafe fn offset_mut<T>(dst: *mut T, n: isize) -> *mut T {
        offset(dst as *const T, n) as *mut T
    }

    pub unsafe fn transmute_copy<T,U>(p: &T) -> U {
        let p = p as *const T as *const U;
        let mut u = uninit();
        copy_nonoverlapping_memory(&mut u, p, 1);
        u
    }
}

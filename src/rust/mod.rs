#![macro_escape]

pub mod macros;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod rand;

pub mod prelude {
    pub use core::slice::{ImmutableSlice, Items, MutableSlice};
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

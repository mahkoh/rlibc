#[cfg(target_arch = "x86_64")]
pub mod x86_64;

extern "rust-intrinsic" {
    pub fn size_of<T>() -> uint;
    pub fn offset<T>(dst: *T, n: int) -> *T;
    pub fn transmute<T,U>(e: T) -> U;
    pub fn uninit<T>() -> T;
    pub fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *T, count: uint);
}

#[lang="sized"]
pub trait Sized { }

#[inline(always)]
pub unsafe fn offset_mut<T>(dst: *mut T, n: int) -> *mut T {
    offset(dst as *T, n) as *mut T
}

pub struct Slice<T> {
    pub data: *T,
    pub len: uint,
}

pub unsafe fn transmute_copy<T,U>(p: &T) -> U {
    let p = p as *T as *U;
    let mut u = uninit();
    copy_nonoverlapping_memory(&mut u, p, 1);
    u
}

pub trait Repr<T> {
    #[inline]
    fn repr(&self) -> T {
        unsafe {
            transmute_copy(self)
        }
    }
}

impl<'a, T> Repr<Slice<T>> for &'a [T] { }
impl<'a> Repr<Slice<u8>> for &'a str { }

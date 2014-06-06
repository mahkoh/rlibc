pub use self::types::*;

pub type char_t   = i8;
pub type short_t  = i16;
pub type ushort_t = u16;
pub type int_t    = i32;
pub type uint_t   = u32;
pub type long_t   = i64;
pub type ulong_t  = u64;

pub type ssize_t = long_t;
pub type size_t  = ulong_t;

pub type intmax_t  = long_t;
pub type uintmax_t = ulong_t;

#[cfg(target_os = "linux")]
#[cfg(target_os = "android")]
mod types {
    pub use self::linux;
    mod linux;
}

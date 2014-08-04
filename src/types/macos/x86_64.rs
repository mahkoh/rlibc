pub type char_t     = i8;
pub type uchar_t    = u8;
pub type short_t    = i16;
pub type ushort_t   = u16;
pub type int_t      = i32;
pub type uint_t     = u32;
pub type long_t     = i64;
pub type longlong_t = i64;
pub type ulong_t    = u64;

pub type ssize_t = long_t;
pub type size_t  = ulong_t;

pub type intmax_t  = long_t;
pub type uintmax_t = ulong_t;

#[repr(u8)]
pub enum void_t {
    __variant1,
    __variant2,
}

// sys/types.h
pub type mode_t = u16;

// kernel
pub type user_addr_t = u64;
pub type user_size_t = u64;
pub type user_ssize_t = i64;

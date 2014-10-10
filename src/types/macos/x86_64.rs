pub type char_t     = i8;
pub type uchar_t    = u8;
pub type short_t    = i16;
pub type ushort_t   = u16;
pub type int_t      = i32;
pub type uint_t     = u32;
pub type long_t     = i64;
pub type longlong_t = i64;
pub type ulong_t    = u64;
pub type ulonglong_t= i64;

// stddef
pub type ssize_t   = long_t;
pub type size_t    = ulong_t;
pub type ptrdiff_t = long_t;

// stdint
pub type int8_t = char_t;
pub type int16_t = short_t;
pub type int32_t = int_t;
pub type int64_t = longlong_t;
pub type uint8_t = uchar_t;
pub type uint16_t = ushort_t;
pub type uint32_t = uint_t;
pub type uint64_t = ulonglong_t;
pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;
pub type intptr_t = long_t;
pub type uintptr_t = ulong_t;
pub type intmax_t  = long_t;
pub type uintmax_t = ulong_t;

// time
pub type clock_t = ulong_t;
pub type time_t  = long_t;
#[repr(C)]
pub struct tm {
    pub tm_sec: int_t,
    pub tm_min: int_t,
    pub tm_hour: int_t,
    pub tm_mday: int_t,
    pub tm_mon: int_t,
    pub tm_year: int_t,
    pub tm_wday: int_t,
    pub tm_yday: int_t,
    pub tm_isdst: int_t,
    pub tm_gmtoff: long_t,
    pub tm_zone: *mut char_t,
}

#[repr(u8)]
pub enum void_t {
    __variant1,
    __variant2,
}

// sys/types.h
pub type dev_t   = int32_t;
pub type gid_t   = uint32_t;
pub type id_t    = uint32_t;
pub type ino_t   = uint64_t;
pub type mode_t  = uint16_t;
pub type nlink_t = uint16_t;
pub type off_t   = int64_t;
pub type pid_t   = int32_t;
pub type uid_t   = uint32_t;

pub type key_t   = int32_t;
pub type useconds_t = uint32_t;
pub type suseconds_t = int32_t;

// sys/time.h
#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: int_t,
    pub tz_dsttime: int_t,
}
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [int32_t, ..32u],
}

// kernel
pub type user_addr_t = u64;
pub type user_size_t = u64;
pub type user_ssize_t = i64;

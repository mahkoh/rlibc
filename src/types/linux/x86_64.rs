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
pub type intmax_t  = long_t;
pub type uintmax_t = ulong_t;

#[repr(u8)]
pub enum void_t {
    __variant1,
    __variant2,
}

pub type __kernel_size_t    = ulong_t;
pub type __kernel_ssize_t   = long_t;
pub type __kernel_ptrdiff_t = long_t;

#[no_mangle]
pub static _NSIG_BPW: uint = 64;

#[packed]
pub struct epoll_event {
    pub events: u32,
    pub data: u64,
}

pub struct stat {
    pub std_dev: ulong_t,
    pub st_ino: ulong_t,
    pub st_nlink: ulong_t,

    pub st_mode: uint_t,
    pub st_uid: uint_t,
    pub st_gid: uint_t,
    pub __pad0: uint_t,
    pub st_rdev:    ulong_t,
    pub st_size:    long_t,
    pub st_blksize: long_t,
    pub st_blocks:  long_t,

    pub st_atime:      ulong_t,
    pub st_atime_nsec: ulong_t,
    pub st_mtime:      ulong_t,
    pub st_mtime_nsec: ulong_t,
    pub st_ctime:      ulong_t,
    pub st_ctime_nsec: ulong_t,
    pub __unused: [long_t, ..3],
}

pub struct iocb {
    pub aio_data: u64,
    pub aio_key: u32,
    pub aio_reserved1: u32,
    pub aio_lio_opcode: u16,
    pub aio_reqprio: i16,
    pub aio_fildes: u32,
    pub aio_buf: u64,
    pub aio_nbytes: u64,
    pub aio_offset: i64,
    pub aio_reserved2: u64,
    pub aio_flags: u32,
    pub aio_resfd: u32,
}

pub struct pt_regs {
        pub r15: ulong_t,
        pub r14: ulong_t,
        pub r13: ulong_t,
        pub r12: ulong_t,
        pub bp: ulong_t,
        pub bx: ulong_t,
        pub r11: ulong_t,
        pub r10: ulong_t,
        pub r9: ulong_t,
        pub r8: ulong_t,
        pub ax: ulong_t,
        pub cx: ulong_t,
        pub dx: ulong_t,
        pub si: ulong_t,
        pub di: ulong_t,
        pub orig_ax: ulong_t,
        pub ip: ulong_t,
        pub cs: ulong_t,
        pub flags: ulong_t,
        pub sp: ulong_t,
        pub ss: ulong_t,
}

pub struct stack_t {
    pub ss_sp: *mut void_t,
    pub ss_flags: int_t,
    pub ss_size: size_t,
}

pub struct cap_user_data_t {
    pub effective: u32,
    pub permitted: u32,
    pub inheritable: u32,
}

pub struct cap_user_header_t {
    pub version: u32,
    pub pid: int_t,
}

pub struct msg {
    pub byte3: u8,
    pub byte2: u8,
    pub eth_id: u8,
    pub cmd: u8,
    pub byte7: u8,
    pub byte6: u8,
    pub byte5: u8,
    pub byte4: u8,
}

pub type __statfs_word = long_t;

#[no_mangle]
pub static __FD_SETSIZE: uint = 1024;
pub struct __kernel_fd_set {
    // XXX size_of
    pub fds_bits: [ulong_t, ..(__FD_SETSIZE / (8 * 8))],
}

pub struct getcpu_cache {
    // XXX size_of
    pub blob: [ulong_t, ..128/8],
}

pub struct sysinfo {
    pub uptime: long_t,
    pub loads: [ulong_t, ..3],
    pub totalram: ulong_t,
    pub freeram: ulong_t,
    pub sharedram: ulong_t,
    pub bufferram: ulong_t,
    pub totalswap: ulong_t,
    pub freeswap: ulong_t,
    pub procs: u16,
    pub pad: u16,
    pub totalhigh: ulong_t,
    pub freehigh: ulong_t,
    pub mem_unit: u32,
    // XXX size_of
    pub _f: [char_t, ..20-2*8-4],
}

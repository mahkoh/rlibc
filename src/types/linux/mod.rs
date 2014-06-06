pub use self::arch::*;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
pub mod arch;

pub type __kernel_key_t       = int_t;
pub type __kernel_long_t      = long_t;
pub type __kernel_ulong_t     = ulong_t;
pub type __kernel_ino_t       = __kernel_ulong_t;
pub type __kernel_mode_t      = uint_t;
pub type __kernel_pid_t       = int_t;
pub type __kernel_ipc_pid_t   = int_t;
pub type __kernel_uid_t       = uint_t;
pub type __kernel_gid_t       = uint_t;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t     = int_t;
pub type __kernel_uid32_t     = uint_t;
pub type __kernel_gid32_t     = uint_t;
pub type __kernel_old_uid32_t = __kernel_uid_t;
pub type __kernel_old_gid32_t = __kernel_gid_t;
pub type __kernel_old_dev_t   = uint_t;
pub struct __kernel_fsid_t {
    pub val: [int_t, ..2],
}
pub type __kernel_off_t       = __kernel_long_t;
pub type __kernel_loff_t      = longlong_t;
pub type __kernel_time_t      = __kernel_long_t;
pub type __kernel_clock_t     = __kernel_long_t;
pub type __kernel_timer_t     = int_t;
pub type __kernel_clockid_t   = int_t;
pub type __kernel_caddr_t     = *mut char_t;
pub type __kernel_uid16_t     = ushort_t;
pub type __kernel_gid16_t     = ushort_t;

pub type pid_t = int_t;
pub type clockid_t = __kernel_clockid_t;

pub struct pollfd {
    pub fd: int_t,
    pub events: short_t,
    pub revents: short_t,
}

pub struct iovec {
    pub iov_base: *mut void_t,
    pub iov_len: __kernel_size_t,
}

pub struct msghdr {
    pub msg_name: *mut void_t,
    pub msg_namelen: int_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: __kernel_size_t,
    pub msg_control: *mut void_t,
    pub msg_controllen: __kernel_size_t,
    pub msg_flags: uint_t,
}

pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: uint_t,
}

pub struct timex {
    pub modes: uint_t, /* mode selector */
    pub offest: __kernel_long_t, /* time offset (usec) */
    pub freq: __kernel_long_t,  /* frequency offset (scaled ppm) */
    pub maxerror: __kernel_long_t, /* maximum error (usec) */
    pub esterror: __kernel_long_t, /* estimated error (usec) */
    pub status: int_t, /* clock command/status */
    pub constant: __kernel_long_t, /* pll time constant */
    pub precision: __kernel_long_t, /* clock precision (usec) (read only) */
    pub tolerance: __kernel_long_t, /* clock frequency tolerance (ppm)
    pub                            * (read only)
    pub                            */
    pub time: timeval, /* (read only, except for ADJ_SETOFFSET) */
    pub tick: __kernel_long_t, /* (modified) usecs between clock ticks */

    pub ppsfreq: __kernel_long_t, /* pps frequency (scaled ppm) (ro) */
    pub jitter: __kernel_long_t, /* pps jitter (us) (ro) */
    pub shift: int_t, /* interval duration (s) (shift) (ro) */
    pub stabil: __kernel_long_t, /* pps stability (scaled ppm) (ro) */
    pub jitcnt: __kernel_long_t, /* jitter limit exceeded (ro) */
    pub calcnt: __kernel_long_t, /* calibration intervals (ro) */
    pub errcnt: __kernel_long_t, /* calibration errors (ro) */
    pub stbcnt: __kernel_long_t, /* stability limit exceeded (ro) */

    pub tai: int_t, /* TAI offset (ro) */

    pub _____padding: [i32, ..11],
}

pub struct timeval {
    pub tv_sec: __kernel_time_t,
    pub tv_usec: __kernel_suseconds_t,
}

pub struct file_handle {
    pub handle_bytes: u32,
    pub handle_type: int_t,
    pub f_handle: [uchar_t, ..0],
}

pub struct rlimit64 {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

pub struct timespec {
    pub tv_sec: __kernel_time_t,
    pub tv_nsec: long_t,
}

#[no_mangle]
pub static _NSIG: uint = 64;
#[no_mangle]
pub static _NSIG_WORDS: uint = _NSIG / _NSIG_BPW;

pub struct sigset_t {
    pub sig: [ulong_t, .._NSIG_WORDS],
}

pub type sa_family_t = __kernel_sa_family_t;

pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [char_t, ..14],
}

pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

pub type loff_t = __kernel_loff_t;

pub struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: long_t,
    pub list_op_pending: *mut robust_list,
}

pub struct robust_list {
    pub next: *mut robust_list,
}

pub type fd_set = __kernel_fd_set;
pub type mode_t = __kernel_mode_t;
pub type uid_t = __kernel_uid32_t;
pub type gid_t = __kernel_gid32_t;

pub type key_serial_t = i32;

pub struct rusage {
    pub ru_utime: timeval,      /* user time used */
    pub ru_stime: timeval,      /* system time used */
    pub ru_maxrss: long_t,      /* maximum resident set size */
    pub ru_ixrss: long_t,       /* integral shared memory size */
    pub ru_idrss: long_t,       /* integral unshared data size */
    pub ru_isrss: long_t,       /* integral unshared stack size */
    pub ru_minflt: long_t,      /* page reclaims */
    pub ru_majflt: long_t,      /* page faults */
    pub ru_nswap: long_t,       /* swaps */
    pub ru_inblock: long_t,     /* block input operations */
    pub ru_oublock: long_t,     /* block output operations */
    pub ru_msgsnd: long_t,      /* messages sent */
    pub ru_msgrcv: long_t,      /* messages received */
    pub ru_nsignals: long_t,    /* signals received */
    pub ru_nvcsw: long_t,       /* voluntary context switches */
    pub ru_nivcsw: long_t,      /* involuntary " */
}

pub struct kexec_segment {
    pub buf: *void_t,
    pub bufsz: size_t,
    pub mem: *void_t,
    pub memsz: size_t,
}

pub struct mq_attr {
    pub mq_flags: long_t,
    pub mq_maxmsg: long_t,
    pub mq_msgsize: long_t,
    pub mq_curmsgs: long_t,
    pub __reserved: [long_t, ..4],
}

pub type mqd_t = __kernel_mqd_t;
pub type timer_t = __kernel_timer_t;

pub struct sembuf {
    sem_num: ushort_t,
    sem_op: short_t,
    sem_flg: short_t,
}

pub struct linux_dirent64 {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: ushort_t,
    pub d_type: uchar_t,
    pub d_name: [char_t, ..0],
}

pub struct io_event {
    pub data: u64,
    pub obj: u64,
    pub res: i64,
    pub res2: i64,
}

pub type aio_context_t = __kernel_ulong_t;
pub type time_t = __kernel_time_t;
pub type qid_t = __kernel_uid32_t;

pub struct timezone {
    pub tz_minuteswest: int_t,
    pub tz_dsttime: int_t,
}

pub struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}

pub struct __sysctl_args {
    pub name: *mut int_t,
    pub nlen: int_t,
    pub oldval: *mut void_t,
    pub oldlenp: *mut size_t,
    pub newval: *mut void_t,
    pub newlen: size_t,
    pub __unused: [ulong_t, ..4],
}

pub struct sched_param {
    pub sched_priority: int_t,
}

pub struct statfs {
    pub f_type: __statfs_word,
    pub f_bsize: __statfs_word,
    pub f_blocks: __statfs_word,
    pub f_bfree: __statfs_word,
    pub f_bavail: __statfs_word,
    pub f_files: __statfs_word,
    pub f_ffree: __statfs_word,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __statfs_word,
    pub f_frsize: __statfs_word,
    pub f_flags: __statfs_word,
    pub f_spare: [__statfs_word, ..4],
}

pub struct ustat {
    pub f_tfree: __kernel_daddr_t,
    pub f_tinode: __kernel_ino_t,
    pub f_fname: [char_t, ..6],
    pub f_fpack: [char_t, ..6],
}

pub struct utimbuf {
    pub actime: __kernel_time_t,
    pub modtime: __kernel_time_t,
}

pub struct linux_dirent {
    pub d_ino: ulong_t,
    pub d_off: ulong_t,
    pub d_reclen: ushort_t,
    pub d_name: [char_t, ..1],
}

pub struct msqid_ds {
    pub msg_perm: ipc_perm,
    pub msg_first: *mut msg,
    pub msg_last: *mut msg,
    pub msg_stime: __kernel_time_t,
    pub msg_rtime: __kernel_time_t,
    pub msg_ctime: __kernel_time_t,
    pub msg_lcbytes: ulong_t,
    pub msg_lqbytes: ulong_t,
    pub msg_cbytes: ushort_t,
    pub msg_lqnum: ushort_t,
    pub msg_qbytes: ushort_t,
    pub msg_lspid: __kernel_ipc_pid_t,
    pub msg_lrpid: __kernel_ipc_pid_t,
}

pub struct msgbuf {
    pub mtype: __kernel_long_t,
    pub mtext: [char_t, ..1],
}

pub type key_t = __kernel_key_t;

pub struct old_utsname {
    pub sysname: [char_t, ..65],
    pub nodename: [char_t, ..65],
    pub release: [char_t, ..65],
    pub version: [char_t, ..65],
    pub machine: [char_t, ..65],
}

pub type off_t = __kernel_off_t;

pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: int_t,
    pub shm_atime: __kernel_time_t,
    pub shm_dtime: __kernel_time_t,
    pub shm_ctime: __kernel_time_t,
    pub shm_cpid: __kernel_ipc_pid_t,
    pub shm_lpid: __kernel_ipc_pid_t,
    pub shm_nattch: ushort_t,
    pub shm_unused: ushort_t,
    pub shm_unused2: *mut void_t,
    pub shm_unused3: *mut void_t,
}

pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: ulong_t,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t,
}

pub type __sigrestore_t = *mut __restorefn_t;
pub type __restorefn_t = fn();

pub type __sighandler_t = *mut __signalfn_t;
pub type __signalfn_t = fn(int_t);

pub struct ipc_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid_t,
    pub gid: __kernel_gid_t,
    pub cuid: __kernel_uid_t,
    pub cgid: __kernel_gid_t,
    pub mode: __kernel_mode_t,
    pub seq: ushort_t,
}

pub type __kernel_mqd_t = int_t;
pub type __kernel_sa_family_t = ushort_t;

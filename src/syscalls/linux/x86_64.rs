#![allow(unused_assignments)]

use types::*;

macro_rules! syscall {
    ($id:expr, $name:ident) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name() -> int_t {
            let mut ret: int_t = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a) -> int_t {
            let mut ret: int_t = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b) -> int_t {
            let mut ret: int_t = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c) -> int_t {
            let mut ret: int_t = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d) -> int_t {
            let mut ret: int_t = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e) -> int_t {
            let mut ret: int_t = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) "{r8}"(e) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e, f:$f) -> int_t {
            let mut ret: int_t = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) "{r8}"(e), "{r9}"(f) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
}

syscall!(000, sys_read, uint_t, *mut char_t, size_t);
syscall!(001, sys_write, uint_t, *const char_t, size_t);
syscall!(002, sys_open, *const char_t, int_t, int_t);
syscall!(003, sys_close, uint_t);
syscall!(004, sys_stat, *const char_t, *mut stat);
syscall!(005, sys_fstat, uint_t, *mut stat);
syscall!(006, sys_lstat, *const char_t, *mut stat);
syscall!(007, sys_poll, *mut pollfd, uint_t, long_t);
syscall!(008, sys_lseek, uint_t, off_t, uint_t);
syscall!(009, sys_mmap, ulong_t, ulong_t, ulong_t, ulong_t, ulong_t, ulong_t);
syscall!(010, sys_mprotect, ulong_t, size_t, ulong_t);
syscall!(011, sys_munmap, ulong_t, size_t);
syscall!(012, sys_brk, ulong_t);
syscall!(013, sys_rt_sigaction, int_t, *const sigaction, *mut sigaction, size_t);
syscall!(014, sys_rt_sigprocmask, int_t, *mut sigset_t, *mut sigset_t, size_t);
syscall!(015, sys_rt_sigreturn, ulong_t);
syscall!(016, sys_ioctl, uint_t, uint_t, ulong_t);
syscall!(017, sys_pread64, ulong_t, *mut char_t, size_t, loff_t);
syscall!(018, sys_pwrite64, uint_t, *const char_t, size_t, loff_t);
syscall!(019, sys_readv, ulong_t, *const iovec, ulong_t);
syscall!(020, sys_writev, ulong_t, *const iovec, ulong_t);
syscall!(021, sys_access, *const char_t, int_t);
syscall!(022, sys_pipe, *mut int_t);
syscall!(023, sys_select, int_t, *mut fd_set, *mut fd_set, *mut fd_set, *mut timeval);
syscall!(024, sys_sched_yield);
syscall!(025, sys_mremap, ulong_t, ulong_t, ulong_t, ulong_t, ulong_t);
syscall!(026, sys_msync, ulong_t, size_t, int_t);
syscall!(027, sys_mincore, ulong_t, size_t, *mut uchar_t);
syscall!(028, sys_madvise, ulong_t, size_t, int_t);
syscall!(029, sys_shmget, key_t, size_t, int_t);
syscall!(030, sys_shmat, int_t, *mut char_t, int_t);
syscall!(031, sys_shmctl, int_t, int_t, *mut shmid_ds);
syscall!(032, sys_dup, uint_t);
syscall!(033, sys_dup2, uint_t, uint_t);
syscall!(034, sys_pause);
syscall!(035, sys_nanosleep, *mut timespec, *mut timespec);
syscall!(036, sys_getitimer, int_t, *mut itimerval);
syscall!(037, sys_alarm, uint_t);
syscall!(038, sys_setitimer, int_t, *mut itimerval, *mut itimerval);
syscall!(039, sys_getpid);
syscall!(040, sys_sendfile, int_t, int_t, *mut off_t, size_t);
syscall!(041, sys_socket, int_t, int_t, int_t);
syscall!(042, sys_connect, int_t, *mut sockaddr, int_t);
syscall!(043, sys_accept, int_t, *mut sockaddr, *mut int_t);
syscall!(044, sys_sendto, int_t, *mut void_t, size_t, uint_t, *mut sockaddr, int_t);
syscall!(045, sys_recvfrom, int_t, *mut void_t, size_t, uint_t, *mut sockaddr, *mut int_t);
syscall!(046, sys_sendmsg, int_t, *mut msghdr, uint_t);
syscall!(047, sys_recvmsg, int_t, *mut msghdr, uint_t);
syscall!(048, sys_shutdown, int_t, int_t);
syscall!(049, sys_bind, int_t, *mut sockaddr, int_t);
syscall!(050, sys_listen, int_t, int_t);
syscall!(051, sys_getsockname, int_t, *mut sockaddr, *mut int_t);
syscall!(052, sys_getpeername, int_t, *mut sockaddr, *mut int_t);
syscall!(053, sys_socketpair, int_t, int_t, int_t, *mut int_t);
syscall!(054, sys_setsockopt, int_t, int_t, int_t, *mut char_t, int_t);
syscall!(055, sys_getsockopt, int_t, int_t, int_t, *mut char_t, *mut int_t);
syscall!(056, sys_clone, ulong_t, ulong_t, *mut void_t, *mut void_t);
syscall!(057, sys_fork);
syscall!(058, sys_vfork);
syscall!(059, sys_execve, *const char_t, *const *const char_t, *const *const char_t);
syscall!(060, sys_exit, int_t);
syscall!(061, sys_wait4, pid_t, *mut int_t, int_t, *mut rusage);
syscall!(062, sys_kill, pid_t, int_t);
syscall!(063, sys_uname, *mut old_utsname);
syscall!(064, sys_semget, key_t, int_t, int_t);
syscall!(065, sys_semop, int_t, *mut sembuf, uint_t);
// TODO syscall!(066, sys_semctl, int_t, int_t, int_t, union semun);
syscall!(067, sys_shmdt, *mut char_t);
syscall!(068, sys_msgget, key_t, int_t);
syscall!(069, sys_msgsnd, int_t, *mut msgbuf, size_t, int_t);
syscall!(070, sys_msgrcv, int_t, *mut msgbuf, size_t, long_t, int_t);
syscall!(071, sys_msgctl, int_t, int_t, *mut msqid_ds);
syscall!(072, sys_fcntl, uint_t, uint_t, ulong_t);
syscall!(073, sys_flock, uint_t, uint_t);
syscall!(074, sys_fsync, uint_t);
syscall!(075, sys_fdatasync, uint_t);
syscall!(076, sys_truncate, *const char_t, long_t);
syscall!(077, sys_ftruncate, uint_t, ulong_t);
syscall!(078, sys_getdents, uint_t, *mut linux_dirent, uint_t);
syscall!(079, sys_getcwd, *mut char_t, ulong_t);
syscall!(080, sys_chdir, *const char_t);
syscall!(081, sys_fchdir, uint_t);
syscall!(082, sys_rename, *const char_t, *const char_t);
syscall!(083, sys_mkdir, *const char_t, int_t);
syscall!(084, sys_rmdir, *const char_t);
syscall!(085, sys_creat, *const char_t, int_t);
syscall!(086, sys_link, *const char_t, *const char_t);
syscall!(087, sys_unlink, *const char_t);
syscall!(088, sys_symlink, *const char_t, *const char_t);
syscall!(089, sys_readlink, *const char_t, *mut char_t, int_t);
syscall!(090, sys_chmod, *const char_t, mode_t);
syscall!(091, sys_fchmod, uint_t, mode_t);
syscall!(092, sys_chown, *const char_t, uid_t, gid_t);
syscall!(093, sys_fchown, uint_t, uid_t, gid_t);
syscall!(094, sys_lchown, *const char_t, uid_t, gid_t);
syscall!(095, sys_umask, int_t);
syscall!(096, sys_gettimeofday, *mut timeval, *mut timezone);
syscall!(097, sys_getrlimit, uint_t, *mut rlimit);
syscall!(098, sys_getrusage, int_t, *mut rusage);
syscall!(099, sys_sysinfo, *mut sysinfo);
syscall!(100, sys_times, *mut sysinfo);
syscall!(101, sys_ptrace, long_t, long_t, ulong_t, ulong_t);
syscall!(102, sys_getuid);
syscall!(103, sys_syslog, int_t, *mut char_t, int_t);
syscall!(104, sys_getgid);
syscall!(105, sys_setuid, uid_t);
syscall!(106, sys_setgid, gid_t);
syscall!(107, sys_geteuid);
syscall!(108, sys_getegid);
syscall!(109, sys_setpgid, pid_t, pid_t);
syscall!(110, sys_getppid);
syscall!(111, sys_getpgrp);
syscall!(112, sys_setsid);
syscall!(113, sys_setreuid, uid_t, uid_t);
syscall!(114, sys_setregid, gid_t, gid_t);
syscall!(115, sys_getgroups, int_t, *mut gid_t);
syscall!(116, sys_setgroups, int_t, *mut gid_t);
syscall!(117, sys_setresuid, *mut uid_t, *mut uid_t, *mut uid_t);
syscall!(118, sys_getresuid, *mut uid_t, *mut uid_t, *mut uid_t);
syscall!(119, sys_setresgid, gid_t, gid_t, gid_t);
syscall!(120, sys_getresgid, *mut gid_t, *mut gid_t, *mut gid_t);
syscall!(121, sys_getpgid, pid_t);
syscall!(122, sys_setfsuid, uid_t);
syscall!(123, sys_setfsgid, gid_t);
syscall!(124, sys_getsid, pid_t);
syscall!(125, sys_capget, cap_user_header_t, cap_user_data_t);
syscall!(126, sys_capset, cap_user_header_t, *const cap_user_data_t);
syscall!(127, sys_rt_sigpending, *mut sigset_t, size_t);
// TODO syscall!(128, sys_rt_sigtimedwait, *sigset_t, *mut siginfo_t, *timespec, size_t);
// TODO syscall!(129, sys_rt_sigqueueinfo, pid_t, int_t, *mut siginfo_t);
syscall!(130, sys_rt_sigsuspend, *mut sigset_t, size_t);
syscall!(131, sys_sigaltstack, *const stack_t, *mut stack_t);
syscall!(132, sys_utime, *mut char_t, *mut utimbuf);
syscall!(133, sys_mknod, *const char_t, int_t, uint_t);
// syscall!(134, sys_uselib, NOT);
syscall!(135, sys_personality, uint_t);
syscall!(136, sys_ustat, uint_t, *mut ustat);
syscall!(137, sys_statfs, *const char_t, *mut statfs);
syscall!(138, sys_fstatfs, uint_t, *mut statfs);
syscall!(139, sys_sysfs, int_t, ulong_t, ulong_t);
syscall!(140, sys_getpriority, int_t, int_t);
syscall!(141, sys_setpriority, int_t, int_t, int_t);
syscall!(142, sys_sched_setparam, pid_t, *mut sched_param);
syscall!(143, sys_sched_getparam, pid_t, *mut sched_param);
syscall!(144, sys_sched_setscheduler, pid_t, int_t, *mut sched_param);
syscall!(145, sys_sched_getscheduler, pid_t);
syscall!(146, sys_sched_get_priority_max, int_t);
syscall!(147, sys_sched_get_priority_min, int_t);
syscall!(148, sys_sched_rr_get_int_terval, pid_t, *mut timespec);
syscall!(149, sys_mlock, ulong_t, size_t);
syscall!(150, sys_munlock, ulong_t, size_t);
syscall!(151, sys_mlockall, int_t);
syscall!(152, sys_munlockall);
syscall!(153, sys_vhangup);
syscall!(154, sys_modify_ldt, int_t, *mut void_t, ulong_t);
syscall!(155, sys_pivot_root, *const char_t, *const char_t);
syscall!(156, sys__sysctl, *mut __sysctl_args);
// This causes llvm to segfault syscall!(157, sys_prctl, int_t, ulong_t, ulong_t, ulong_t, ();, ulong_t)
// TODO syscall!(158, sys_arch_prctl, *mut task_struct, int_t, *mut ulong_t);
syscall!(159, sys_adjtimex, *mut timex);
syscall!(160, sys_setrlimit, uint_t, *mut rlimit);
syscall!(161, sys_chroot, *const char_t);
syscall!(162, sys_sync);
syscall!(163, sys_acct, *const char_t);
syscall!(164, sys_settimeofday, *mut timeval, *mut timezone);
syscall!(165, sys_mount, *mut char_t, *mut char_t, *mut char_t, ulong_t, *mut void_t);
syscall!(166, sys_umount2, *const char_t, int_t);
syscall!(167, sys_swapon, *const char_t, int_t);
syscall!(168, sys_swapoff, *const char_t);
syscall!(169, sys_reboot, int_t, int_t, uint_t, *mut void_t);
syscall!(170, sys_sethostname, *mut char_t, int_t);
syscall!(171, sys_setdomainname, *mut char_t, int_t);
syscall!(172, sys_iopl, uint_t, *mut pt_regs);
syscall!(173, sys_ioperm, ulong_t, ulong_t, int_t);
// syscall!(174, sys_create_module, REMOVED IN Linux 2.);
syscall!(175, sys_init_module, *mut void_t, ulong_t, *const char_t);
syscall!(176, sys_delete_module, *const char_t, uint_t);
// syscall!(177, sys_get_kernel_syms, REMOVED IN Linux 2.);
// syscall!(178, sys_query_module, REMOVED IN Linux 2.);
syscall!(179, sys_quotactl, uint_t, *const char_t, qid_t, *mut void_t);
// syscall!(180, sys_nfsservctl, NOT);
// syscall!(181, sys_getpmsg, NOT);
// syscall!(182, sys_putpmsg, NOT);
// syscall!(183, sys_afs_syscall, NOT);
// syscall!(184, sys_tuxcall, NOT);
// syscall!(185, sys_security, NOT);
syscall!(186, sys_gettid);
syscall!(187, sys_readahead, int_t, loff_t, size_t);
syscall!(188, sys_setxattr, *const char_t, *const char_t, *const void_t, size_t, int_t);
syscall!(189, sys_lsetxattr, *const char_t, *const char_t, *const void_t, size_t, int_t);
syscall!(190, sys_fsetxattr, int_t, *const char_t, *const void_t, size_t, int_t);
syscall!(191, sys_getxattr, *const char_t, *const char_t, *mut void_t, size_t);
syscall!(192, sys_lgetxattr, *const char_t, *const char_t, *mut void_t, size_t);
syscall!(193, sys_fgetxattr, int_t, *const char_t, *mut void_t, size_t);
syscall!(194, sys_listxattr, *const char_t, *mut char_t, size_t);
syscall!(195, sys_llistxattr, *const char_t, *mut char_t, size_t);
syscall!(196, sys_flistxattr, int_t, *mut char_t, size_t);
syscall!(197, sys_removexattr, *const char_t, *const char_t);
syscall!(198, sys_lremovexattr, *const char_t, *const char_t);
syscall!(199, sys_fremovexattr, int_t, *const char_t);
// TODO syscall!(200, sys_tkill, pid_t, ing);
syscall!(201, sys_time, *mut time_t);
syscall!(202, sys_futex, *mut u32, int_t, u32, *mut timespec, *mut u32, u32);
syscall!(203, sys_sched_setaffinity, pid_t, uint_t, *mut ulong_t);
syscall!(204, sys_sched_getaffinity, pid_t, uint_t, *mut ulong_t);
// syscall!(205, sys_set_thread_area, NOT IMPLEMENTED. Use);
syscall!(206, sys_io_setup, uint_t, *mut aio_context_t);
syscall!(207, sys_io_destroy, aio_context_t);
syscall!(208, sys_io_getevents, aio_context_t, long_t, long_t, *mut io_event);
syscall!(209, sys_io_submit, aio_context_t, long_t, *mut *mut iocb);
syscall!(210, sys_io_cancel, aio_context_t, *mut iocb, *mut *mut io_event);
// syscall!(211, sys_get_thread_area, NOT IMPLEMENTED. Use);
syscall!(212, sys_lookup_dcookie, u64, long_t, long_t);
syscall!(213, sys_epoll_create, int_t);
// syscall!(214, sys_epoll_ctl_old, NOT);
// syscall!(215, sys_epoll_wait_old, NOT);
syscall!(216, sys_remap_file_pages, ulong_t, ulong_t, ulong_t, ulong_t, ulong_t);
syscall!(217, sys_getdents64, uint_t, *mut linux_dirent64, uint_t);
syscall!(218, sys_set_tid_address, *mut int_t);
syscall!(219, sys_restart_syscall);
syscall!(220, sys_semtimedop, int_t, *mut sembuf, uint_t, *const timespec);
syscall!(221, sys_fadvise64, int_t, loff_t, size_t, int_t);
// TODO syscall!(222, sys_timer_create, *clockid_t, *mut sigevent, *mut timer_t);
syscall!(223, sys_timer_settime, timer_t, int_t, *const itimerspec, *mut itimerspec);
syscall!(224, sys_timer_gettime, timer_t, *mut itimerspec);
syscall!(225, sys_timer_getoverrun, timer_t);
syscall!(226, sys_timer_delete, timer_t);
syscall!(227, sys_clock_settime, *const clockid_t, *const timespec);
syscall!(228, sys_clock_gettime, *const clockid_t, *mut timespec);
syscall!(229, sys_clock_getres, *const clockid_t, *mut timespec);
syscall!(230, sys_clock_nanosleep, *const clockid_t, int_t, *const timespec, *mut timespec);
syscall!(231, sys_exit_group, int_t);
syscall!(232, sys_epoll_wait, int_t, *mut epoll_event, int_t, int_t);
syscall!(233, sys_epoll_ctl, int_t, int_t, int_t, *mut epoll_event);
syscall!(234, sys_tgkill, pid_t, pid_t, int_t);
syscall!(235, sys_utimes, *const char_t, *mut timeval); // WARNING *mut char_t
// syscall!(236, sys_vserver, NOT);
syscall!(237, sys_mbind, ulong_t, ulong_t, ulong_t, *mut ulong_t, ulong_t, uint_t);
syscall!(238, sys_set_mempolicy, int_t, *mut ulong_t, ulong_t);
syscall!(239, sys_get_mempolicy, *mut int_t, *mut ulong_t, ulong_t, ulong_t, ulong_t);
syscall!(240, sys_mq_open, *const char_t, int_t, mode_t, *mut mq_attr);
syscall!(241, sys_mq_unlink, *const char_t);
syscall!(242, sys_mq_timedsend, mqd_t, *const char_t, size_t, uint_t, *const timespec);
syscall!(243, sys_mq_timedreceive, mqd_t, *mut char_t, size_t, *mut uint_t, *const timespec);
// TODO syscall!(244, sys_mq_notify, mqd_t, *sigevent);
syscall!(245, sys_mq_getsetattr, mqd_t, *const mq_attr, *mut mq_attr);
syscall!(246, sys_kexec_load, ulong_t, ulong_t, *mut kexec_segment, ulong_t);
// TODO syscall!(247, sys_waitid, int_t, pid_t, *mut siginfo, int_t, *mut rusage);
syscall!(248, sys_add_key, *const char_t, *const char_t, *const void_t, size_t);
syscall!(249, sys_request_key, *const char_t, *const char_t, *const char_t, key_serial_t);
syscall!(250, sys_keyctl, int_t, ulong_t, ulong_t, ulong_t, ulong_t);
syscall!(251, sys_ioprio_set, int_t, int_t, int_t);
syscall!(252, sys_ioprio_get, int_t, int_t);
syscall!(253, sys_inotify_init);
syscall!(254, sys_inotify_add_watch, int_t, *const char_t, u32);
syscall!(255, sys_inotify_rm_watch, int_t, i32);
syscall!(256, sys_migrate_pages, pid_t, ulong_t, *const ulong_t, *const ulong_t);
syscall!(257, sys_openat, int_t, *const char_t, int_t, int_t);
syscall!(258, sys_mkdirat, int_t, *const char_t, int_t);
syscall!(259, sys_mknodat, int_t, *const char_t, int_t, uint_t);
syscall!(260, sys_fchownat, int_t, *const char_t, uid_t, gid_t, int_t);
syscall!(261, sys_futimesat, int_t, *const char_t, *mut timeval);
syscall!(262, sys_newfstatat, int_t, *const char_t, *mut stat, int_t);
syscall!(263, sys_unlinkat, int_t, *const char_t, int_t);
syscall!(264, sys_renameat, int_t, *const char_t, int_t, *const char_t);
syscall!(265, sys_linkat, int_t, *const char_t, int_t, *const char_t, int_t);
syscall!(266, sys_symlinkat, *const char_t, int_t, *const char_t);
syscall!(267, sys_readlinkat, int_t, *const char_t, *mut char_t, int_t);
syscall!(268, sys_fchmodat, int_t, *const char_t, mode_t);
syscall!(269, sys_faccessat, int_t, *const char_t, int_t);
syscall!(270, sys_pselect6, int_t, *mut fd_set, *mut fd_set, *mut fd_set, *mut timespec,
         *mut void_t);
syscall!(271, sys_ppoll, *mut pollfd, uint_t, *mut timespec, *const sigset_t, size_t);
syscall!(272, sys_unshare, ulong_t);
syscall!(273, sys_set_robust_list, *mut robust_list_head, size_t);
syscall!(274, sys_get_robust_list, int_t, *mut *mut robust_list_head, *mut size_t);
syscall!(275, sys_splice, int_t, *mut loff_t, int_t, *mut loff_t, size_t, uint_t);
syscall!(276, sys_tee, int_t, int_t, size_t, uint_t);
syscall!(277, sys_sync_file_range, long_t, loff_t, loff_t, long_t);
syscall!(278, sys_vmsplice, int_t, *const iovec, ulong_t, uint_t);
syscall!(279, sys_move_pages, pid_t, ulong_t, *mut *const void_t, *const int_t, *mut int_t, int_t);
syscall!(280, sys_utimensat, int_t, *const char_t, *mut timespec, int_t);
syscall!(281, sys_epoll_pwait, int_t, *mut epoll_event, int_t, int_t, *const sigset_t, size_t);
syscall!(282, sys_signalfd, int_t, *mut sigset_t, size_t);
syscall!(283, sys_timerfd_create, int_t, int_t);
syscall!(284, sys_eventfd, uint_t);
syscall!(285, sys_fallocate, long_t, long_t, loff_t, loff_t);
syscall!(286, sys_timerfd_settime, int_t, int_t, *const itimerspec, *mut itimerspec);
syscall!(287, sys_timerfd_gettime, int_t, *mut itimerspec);
syscall!(288, sys_accept4, int_t, *mut sockaddr, *mut int_t, int_t);
syscall!(289, sys_signalfd4, int_t, *mut sigset_t, size_t, int_t);
syscall!(290, sys_eventfd2, uint_t, int_t);
syscall!(291, sys_epoll_create1, int_t);
syscall!(292, sys_dup3, uint_t, uint_t, int_t);
syscall!(293, sys_pipe2, *mut int_t, int_t);
syscall!(294, sys_inotify_init1, int_t);
syscall!(295, sys_preadv, ulong_t, *const iovec, ulong_t, ulong_t, ulong_t);
syscall!(296, sys_pwritev, ulong_t, *const iovec, ulong_t, ulong_t, ulong_t);
// TODO syscall!(297, sys_rt_tgsigqueueinfo, pid_t, pid_t, int_t, *mut siginfo_t);
// TODO syscall!(298, sys_perf_event_open, *mut perf_event_attr, pid_t, int_t, int_t, ulong_t);
syscall!(299, sys_recvmmsg, int_t, *mut msghdr, uint_t, uint_t, *mut timespec);
syscall!(300, sys_fanotify_init, uint_t, uint_t);
syscall!(301, sys_fanotify_mark, long_t, long_t, u64, long_t, long_t);
syscall!(302, sys_prlimit64, pid_t, uint_t, *const rlimit64, *mut rlimit64);
syscall!(303, sys_name_to_handle_at, int_t, *const char_t, *mut file_handle, *mut int_t, int_t);
syscall!(304, sys_open_by_handle_at, int_t, *const char_t, *mut file_handle, *mut int_t, int_t);
syscall!(305, sys_clock_adjtime, clockid_t, *mut timex);
syscall!(306, sys_syncfs, int_t);
syscall!(307, sys_sendmmsg, int_t, *mut mmsghdr, uint_t, uint_t);
syscall!(308, sys_setns, int_t, int_t);
syscall!(309, sys_getcpu, *mut uint_t, *mut uint_t, *mut getcpu_cache);
syscall!(310, sys_process_vm_readv, pid_t, *const iovec, ulong_t, *const iovec, ulong_t, ulong_t);
syscall!(311, sys_process_vm_writev, pid_t, *const iovec, ulong_t, *const iovec, ulong_t, ulong_t);

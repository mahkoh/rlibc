use types::*;

macro_rules! syscall {
    ($id:expr, $name:ident) => {
        #[inline(always)]
        #[no_mangle]
        #[no_split_stack]
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
        #[no_split_stack]
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
        #[no_split_stack]
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
        #[no_split_stack]
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
        #[no_split_stack]
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
        #[no_split_stack]
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
        #[no_split_stack]
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

syscall!(000, sys_nosys)
syscall!(001, sys_exit, int_t) // -> void
syscall!(002, sys_fork)
syscall!(003, sys_read, uint_t, *mut char_t, size_t) // sizeof(*mut) == 8
syscall!(004, sys_write, uint_t, *const char_t, size_t) // uint_t/int_t ???
syscall!(005, sys_open, *const char_t, int_t, int_t)
syscall!(006, sys_close, uint_t)

syscall!(010, sys_unlink, *const char_t)

syscall!(128, sys_rename, *const char_t, *const char_t)

syscall!(137, sys_rmdir, *const char_t)

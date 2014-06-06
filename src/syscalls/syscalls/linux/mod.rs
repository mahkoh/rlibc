pub use self::syscalls::*;

#[cfg(target_arch = "x86_64")]
mod syscalls {
    pub use self::x86_64::*;
    mod x86_64;
}

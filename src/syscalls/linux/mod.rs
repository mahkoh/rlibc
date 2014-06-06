pub use self::syscalls::*;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod syscalls;

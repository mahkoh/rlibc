pub use self::syscalls::*;

#[cfg(target_os = "linux")]
#[cfg(target_os = "android")]
#[path = "linux/mod.rs"]
mod syscalls;

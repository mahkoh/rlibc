pub use self::syscalls::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "linux/mod.rs"]
mod syscalls;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "macos/mod.rs"]
mod syscalls;

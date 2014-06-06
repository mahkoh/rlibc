pub use self::syscalls::*;

#[cfg(target_os = "linux")]
#[cfg(target_os = "android")]
mod syscalls {
    pub use self::linux::*;
    mod linux;
}

pub use self::types::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "linux/mod.rs"]
mod types;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "macos/mod.rs"]
mod types;

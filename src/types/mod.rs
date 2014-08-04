pub use self::types::*;

#[cfg(target_os = "linux")]
#[cfg(target_os = "android")]
#[path = "linux/mod.rs"]
mod types;

#[cfg(target_os = "macos")]
#[cfg(target_os = "ios")]
#[path = "macos/mod.rs"]
mod types;

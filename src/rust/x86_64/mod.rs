#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod macos;

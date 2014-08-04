#[cfg(target_os = "linux")]
#[cfg(target_os = "android")]
pub mod linux;

#[cfg(target_os = "macos")]
#[cfg(target_os = "ios")]
pub mod macos;

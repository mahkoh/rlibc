pub use self::types::*;

#[cfg(target_arch = "x86_64")]
mod types {
    pub use self::x86_64::*;
    mod x86_64;
}

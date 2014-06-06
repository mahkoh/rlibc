#[cfg(target_arch = "x86_64")]
pub mod x86_64;

extern "rust-intrinsic" {
    pub fn size_of<T>() -> uint;
}

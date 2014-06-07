use types::{int_t};

pub static EISDIR: int_t = 21;
pub static EINVAL: int_t = 22;

#[no_mangle]
pub static mut errno: int_t = 0;

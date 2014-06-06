use types::{int_t};

pub static EISDIR: int_t = 21;

#[no_mangle]
pub static mut errno: int_t = 0;

use types::{char_t, int_t};

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn mkstemp(tplt: *mut char_t) -> int_t {
    0
}

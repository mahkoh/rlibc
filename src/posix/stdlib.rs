//use internal::utils::{ToSlice, LastN, Some};
use types::{char_t, int_t};
use libc::string::{strlen};
use libc::errno::{errno, EINVAL};

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn mkstemp(tplt: *mut char_t) -> int_t {
    /*
    let slc = tplt.to_slice(strlen(tplt));
    if slc.len() < 6 {
        errno = EINVAL;
        return -1;
    }
    for c in slc.lastn(6) {
        if c != 'X' as char_t {
            errno = EINVAL;
            return -1;
        }
    }
    */
    0
}

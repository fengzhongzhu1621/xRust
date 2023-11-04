use crate::ffi::win::GlobalUnlock;
use crate::ffi::types::c_void;

#[inline]
pub fn unlock_data(data: *mut c_void) {
    unsafe {
        GlobalUnlock(data);
    }
}

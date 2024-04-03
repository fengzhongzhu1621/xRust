use super::system::GlobalUnlock;
use crate::platform::types::c_void;

#[inline]
pub fn unlock_data(data: *mut c_void) {
    unsafe {
        GlobalUnlock(data);
    }
}

use crate::ffi::types::*;
use crate::ffi::win::sys::*;
use core::ptr;

#[inline(always)]
pub fn free_dc(data: HDC) {
    unsafe {
        ReleaseDC(ptr::null_mut(), data);
    }
}

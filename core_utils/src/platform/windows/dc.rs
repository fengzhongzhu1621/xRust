use super::system::*;
use super::types::*;
use core::ptr;

#[inline(always)]
pub fn free_dc(data: HDC) {
    unsafe {
        ReleaseDC(ptr::null_mut(), data);
    }
}

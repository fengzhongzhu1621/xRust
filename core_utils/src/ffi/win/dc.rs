use crate::ffi::win::SysResult;
use crate::ffi::types::*;
use crate::ffi::win::sys::*;
use error_code::ErrorCode;
use core::ptr;
use core::num::{NonZeroUsize, NonZeroU32};

#[inline(always)]
fn free_dc(data: HDC) {
    unsafe {
        ReleaseDC(ptr::null_mut(), data);
    }
}

use crate::SysResult;
use core_utils::ffi::types::HWND;
use crate::sys::*;
use error_code::ErrorCode;
use core::{slice, mem, ptr, cmp};
use core::num::{NonZeroUsize, NonZeroU32};
use crate::raw::*;

pub struct Clipboard {
    _dummy: ()
}

impl Clipboard {
    #[inline(always)]
    ///Attempts to open clipboard, returning clipboard instance on success.
    pub fn new() -> SysResult<Self> {
        open().map(|_| Self { _dummy: () })
    }


}

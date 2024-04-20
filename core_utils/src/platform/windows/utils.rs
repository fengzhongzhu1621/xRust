use crate::platform::types::*;
use error_code::ErrorCode;
use std::{
    ffi::{OsStr, OsString},
    mem::{self, MaybeUninit},
    os::windows::ffi::{OsStrExt, OsStringExt},
    ptr,
};

#[cold]
#[inline(never)]
/// 返回默认值
pub fn unlikely_empty_size_result<T: Default>() -> T {
    Default::default()
}

#[cold]
#[inline(never)]
/// 返回错误
pub fn unlikely_last_error() -> ErrorCode {
    ErrorCode::last_system()
}

#[inline]
/// 空操作
pub fn noop(_: *mut c_void) {}

fn to_wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}

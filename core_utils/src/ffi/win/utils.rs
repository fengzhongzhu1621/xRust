use crate::ffi::types::c_void;
use error_code::ErrorCode;

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
pub fn noop(_: *mut c_void) {
}

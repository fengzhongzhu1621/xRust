use super::types::*;
use error_code::ErrorCode;
use super::sys;

const BYTES_LAYOUT: alloc::alloc::Layout = alloc::alloc::Layout::new::<u8>();

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

#[inline]
/// 使用全局分配器释放内存。
/// 此函数将调用转发到用 #[global_allocator] 属性注册的分配器的 GlobalAlloc::dealloc 方法。
pub fn free_rust_mem(data: *mut c_void) {
    unsafe {
        alloc::alloc::dealloc(data as _, BYTES_LAYOUT)
    }
}

#[inline]
pub fn free_global_mem(data: *mut c_void) {
    unsafe {
        sys::GlobalFree(data);
    }
}

#[inline]
pub fn unlock_data(data: *mut c_void) {
    unsafe {
        sys::GlobalUnlock(data);
    }
}

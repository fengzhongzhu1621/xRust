use crate::ffi::types::c_void;
use crate::ffi::win::GlobalFree;

const BYTES_LAYOUT: alloc::alloc::Layout = alloc::alloc::Layout::new::<u8>();

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
        GlobalFree(data);
    }
}

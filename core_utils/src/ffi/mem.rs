use super::utils::*;
use super::types::*;
use super::SysResult;
use super::sys;
use error_code::ErrorCode;
use core::{mem, ptr};

pub struct Scope<T: Copy>(pub T, pub fn(T));

impl<T: Copy> Drop for Scope<T> {
    #[inline(always)]
    /// 通用的析构函数
    fn drop(&mut self) {
        (self.1)(self.0)
    }
}

pub struct RawMem(Scope<*mut c_void>);

impl RawMem {
    #[inline(always)]
    pub fn new_rust_mem(size: usize) -> SysResult<Self> {
        // 分配内存
        let mem = unsafe {
            alloc::alloc::alloc_zeroed(alloc::alloc::Layout::array::<u8>(size).expect("To create layout for bytes"))
        };

        if mem.is_null() {
            Err(unlikely_last_error())
        } else {
            Ok(Self(Scope(mem as _, free_rust_mem)))
        }
    }

    #[inline(always)]
    pub fn new_global_mem(size: usize) -> SysResult<Self> {
        unsafe {
            let mem = sys::GlobalAlloc(GHND, size as _);
            if mem.is_null() {
                Err(unlikely_last_error())
            } else {
                Ok(Self(Scope(mem, free_global_mem)))
            }
        }
    }

    #[inline(always)]
    pub fn from_borrowed(ptr: ptr::NonNull<c_void>) -> Self {
        Self(Scope(ptr.as_ptr(), noop))
    }

    #[inline(always)]
    pub fn get(&self) -> *mut c_void {
        // self.0 返回Scope对象
        (self.0).0
    }

    #[inline(always)]
    pub fn release(self) {
        // 获取传给它的值，销毁对象，但是不调用它的析构函数。
        // 即 回收变量占用的空间，但不要关闭底层系统资源
        // 当底层资源的所有权先前已转移到 Rust 之外的代码时 (例如，通过将原始文件描述符传输到 C 代码)，这很有用。
        mem::forget(self)
    }

    pub fn lock(&self) -> SysResult<(ptr::NonNull<c_void>, Scope<*mut c_void>)> {
        let ptr = unsafe {
            sys::GlobalLock(self.get())
        };

        match ptr::NonNull::new(ptr) {
            Some(ptr) => Ok((ptr, Scope(self.get(), unlock_data))),
            None => Err(ErrorCode::last_system()),
        }
    }

}

use super::OsStr;
use core::{ptr::NonNull, slice};

/// Owned allocation of an OS-native string.
/// 尽量使用 NonNull 来包装 *mut T。
/// 非空指针。会自动检查包装的指针是否为空。
pub struct OsString {
    pub alloc: NonNull<libc::c_char>, // 字符串的地址
    /// Length without the nul-byte.
    pub len: usize, // 字符串的长度，不包含空字符
}

/// 实现Send的类型可以在线程间安全的传递其所有权
unsafe impl Send for OsString {}

impl Drop for OsString {
    fn drop(&mut self) {
        // 释放内存
        let ptr = self.alloc.as_ptr() as *mut libc::c_void;
        unsafe { libc::free(ptr) }
    }
}

// OsString -> &OsStr
// 转换后指向的是同一个指针
impl AsRef<OsStr> for OsString {
    fn as_ref(&self) -> &OsStr {
        unsafe {
            // 先转换为切片，然后转换为&OsStr格式
            // 和OsString指向同一块内存空间
            OsStr::from_slice(slice::from_raw_parts(
                self.alloc.as_ptr(),
                self.len,
            ))
        }
    }
}

use super::{Error, OsString};
use crate::ffi::string::IntoOsString;
use core::{fmt, mem::transmute, ptr::NonNull, slice, str};

/// Borrowed allocation of an OS-native string.
#[repr(transparent)]
pub struct OsStr {
    pub bytes: [libc::c_char], // 字节数组
}

impl OsStr {
    /// Unsafe cause sequence needs to end with 0.
    /// mem::transmute<T, U> 接受一个 T 类型的值，然后将它重新解析为类型 U。要求 T 和 U 的大小一样。
    pub unsafe fn from_slice(slice: &[libc::c_char]) -> &Self {
        transmute(slice)
    }
}

impl fmt::Debug for OsStr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut first = false;
        write!(fmt, "[")?;

        for &signed in &self.bytes {
            let byte = signed as u8;
            if first {
                first = false;
            } else {
                write!(fmt, ", ")?;
            }
            if byte.is_ascii() {
                write!(fmt, "{:?}", char::from(byte))?;
            } else {
                write!(fmt, "'\\x{:x}'", byte)?;
            }
        }

        write!(fmt, "]")?;
        Ok(())
    }
}
impl fmt::Display for OsStr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let ptr = self.bytes.as_ptr();
        let len = self.bytes.len();
        let slice = unsafe { slice::from_raw_parts(ptr as _, len) };

        let mut sub = slice;

        while sub.len() > 0 {
            match str::from_utf8(sub) {
                Ok(string) => {
                    write!(fmt, "{}", string)?;
                    sub = &[];
                }
                Err(err) => {
                    let string = str::from_utf8(&sub[..err.valid_up_to()])
                        .expect("Inconsistent utf8 error");
                    write!(fmt, "{}�", string,)?;

                    sub = &sub[err.valid_up_to() + 1..];
                }
            }
        }

        Ok(())
    }
}

// &OsStr -> OsString
// 转换后指向的是不同的地址
impl<'str> IntoOsString for &'str OsStr {
    fn into_os_string(self) -> Result<OsString, Error> {
        // 获得非空字节数组的长度
        let len = self.bytes.len();
        // 需要分配多一个字节的空间，存放\0
        let alloc = unsafe { libc::malloc(len + 1) };
        // 转换为非空指针类型
        let alloc = match NonNull::new(alloc as *mut libc::c_char) {
            Some(alloc) => alloc,
            None => {
                return Err(Error::last_os_error());
            }
        };
        // 拷贝字符串
        unsafe {
            libc::memcpy(
                alloc.as_ptr() as *mut libc::c_void,
                self.bytes.as_ptr() as *const libc::c_void,
                len + 1,
            );
        }

        Ok(OsString { alloc, len })
    }
}

use super::{Error, IntoOsString};
use core::{fmt, mem::transmute, ptr::NonNull, slice, str};

/// Owned allocation of an OS-native string.
pub struct OsString {
    alloc: NonNull<libc::c_char>,
    /// Length without the nul-byte.
    len: usize,
}

unsafe impl Send for OsString {}

impl Drop for OsString {
    fn drop(&mut self) {
        let ptr = self.alloc.as_ptr() as *mut libc::c_void;
        unsafe { libc::free(ptr) }
    }
}

impl AsRef<OsStr> for OsString {
    fn as_ref(&self) -> &OsStr {
        unsafe {
            OsStr::from_slice(slice::from_raw_parts(
                self.alloc.as_ptr(),
                self.len,
            ))
        }
    }
}

/// Borrowed allocation of an OS-native string.
#[repr(transparent)]
pub struct OsStr {
    bytes: [libc::c_char],
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

impl<'str> IntoOsString for &'str OsStr {
    fn into_os_string(self) -> Result<OsString, Error> {
        let len = self.bytes.len();
        let alloc = unsafe { libc::malloc(len + 1) };
        let alloc = match NonNull::new(alloc as *mut libc::c_char) {
            Some(alloc) => alloc,
            None => {
                return Err(Error::last_os_error());
            }
        };
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

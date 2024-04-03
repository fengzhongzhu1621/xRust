use core::{
    ptr::{self, NonNull},
    slice,
};

pub type IOError = std::io::Error;

#[cfg(feature = "std")]
/// An IO error.
pub type Error = std::io::Error;

#[cfg(not(feature = "std"))]
#[derive(Debug)]
/// An IO error. Without std, you can only get a message or an OS error code.
pub struct Error {
    code: i32,
}

#[cfg(not(feature = "std"))]
impl Error {
    /// Creates an error from a raw OS error code.
    pub fn from_raw_os_error(code: i32) -> Self {
        Self { code }
    }

    /// Creates an error from the last OS error code.
    pub fn last_os_error() -> Error {
        Self::from_raw_os_error(unsafe { GetLastError() } as i32)
    }

    /// Raw OS error code. Returns option for compatibility with std.
    pub fn raw_os_error(&self) -> Option<i32> {
        Some(self.code)
    }
}

#[cfg(not(feature = "std"))]
impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut buf: LPWSTR = ptr::null_mut();
        let res = unsafe {
            FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                ptr::null_mut(),
                self.code as DWORD,
                LANG_USER_DEFAULT as DWORD,
                &mut buf as *mut LPWSTR as LPWSTR,
                0,
                ptr::null_mut(),
            )
        };

        if res == 0 {
            write!(fmt, "error getting error message")?;
        } else {
            {
                let slice = unsafe {
                    slice::from_raw_parts(buf as *const WCHAR, res as usize)
                };
                let str = unsafe { OsStr::from_slice(slice) };
                write!(fmt, "{}", str)?;
            }
            unsafe {
                LocalFree(buf as LPVOID);
            }
        }

        Ok(())
    }
}

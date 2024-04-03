use super::error::Error;
use super::os_str::OsStr;
use super::os_string::OsString;
use crate::platform::string::{EitherOsStr, ToOsStr};
use core::{mem::transmute, ptr::NonNull, str};

impl ToOsStr for str {
    fn to_os_str(&self) -> Result<EitherOsStr, Error> {
        make_os_str(self.as_bytes())
    }
}

#[cfg(feature = "std")]
use std::{ffi, os::unix::ffi::OsStrExt};

#[cfg(feature = "std")]
impl ToOsStr for ffi::OsStr {
    fn to_os_str(&self) -> Result<EitherOsStr, Error> {
        make_os_str(self.as_bytes())
    }
}

/// Path must not contain a nul-byte in the middle, but a nul-byte in the end
/// (and only in the end) is allowed, which in this case no extra allocation
/// will be made. Otherwise, an extra allocation is made.
fn make_os_str(slice: &[u8]) -> Result<EitherOsStr, Error> {
    if let Some((&last, init)) = slice.split_last() {
        if init.contains(&0) {
            panic!("Path to file cannot contain nul-byte in the middle");
        }
        if last == 0 {
            // &[u8] -> &OsStr
            let str = unsafe { OsStr::from_slice(transmute(slice)) };
            return Ok(EitherOsStr::Borrowed(str));
        }
    }

    let alloc = unsafe { libc::malloc(slice.len() + 1) };
    let alloc = match NonNull::new(alloc as *mut libc::c_char) {
        Some(alloc) => alloc,
        None => {
            return Err(Error::last_os_error());
        }
    };
    unsafe {
        libc::memcpy(
            alloc.as_ptr() as *mut libc::c_void,
            slice.as_ptr() as *const libc::c_void,
            slice.len(),
        );
        *alloc.as_ptr().add(slice.len()) = 0;
    }

    Ok(EitherOsStr::Owned(OsString { alloc, len: slice.len() }))
}

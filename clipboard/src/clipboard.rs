#![allow(dead_code)]
use core_utils::ffi::win::*;
use core_utils::ffi::types::*;
use super::formats::{Unicode, Setter, Getter};

pub struct Clipboard {
    _dummy: ()
}

impl Clipboard {
    #[inline(always)]
    ///Attempts to open clipboard, returning clipboard instance on success.
    pub fn new() -> SysResult<Self> {
        clipboard::open().map(|_| Self { _dummy: () })
    }

    #[inline(always)]
    ///Attempts to open clipboard, associating it with specified `owner` and returning clipboard instance on success.
    pub fn new_for(owner: HWND) -> SysResult<Self> {
        clipboard::open_for(owner).map(|_| Self { _dummy: () })
    }

    #[inline(always)]
    ///Attempts to open clipboard, giving it `num` retries in case of failure.
    pub fn new_attempts(num: usize) -> SysResult<Self> {
        Self::new_attempts_for(core::ptr::null_mut(), num)
    }

    #[inline]
    ///Attempts to open clipboard, giving it `num` retries in case of failure.
    pub fn new_attempts_for(owner: HWND, mut num: usize) -> SysResult<Self> {
        loop {
            match Self::new_for(owner) {
                Ok(this) => break Ok(this),
                Err(err) => match num {
                    0 => break Err(err),
                    _ => num -= 1,
                }
            }

            //0 causes to yield remaining time in scheduler, but remain to be scheduled once again.
            unsafe { Sleep(0) };
        }
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        let _ = clipboard::close();
    }
}

#[inline(always)]
///Runs provided callable with open clipboard, returning whether clipboard was open successfully.
///
///If clipboard fails to open, callable is not invoked.
pub fn with_clipboard<F: FnMut()>(mut cb: F) -> SysResult<()> {
    let _clip = Clipboard::new()?;
    cb();
    Ok(())
}

#[inline(always)]
///Runs provided callable with open clipboard, returning whether clipboard was open successfully.
///
///If clipboard fails to open, attempts `num` number of retries before giving up.
///In which case closure is not called
pub fn with_clipboard_attempts<F: FnMut()>(num: usize, mut cb: F) -> SysResult<()> {
    let _clip = Clipboard::new_attempts(num)?;
    cb();
    Ok(())
}

#[inline(always)]
///Retrieve data from clipboard.
pub fn get<R: Default, T: Getter<R>>(format: T) -> SysResult<R> {
    let mut result = R::default();
    format.read_clipboard(&mut result).map(|_| result)
}

#[inline(always)]
///Shortcut to retrieve data from clipboard.
///
///It opens clipboard and gets output, if possible.
pub fn get_clipboard<R: Default, T: Getter<R>>(format: T) -> SysResult<R> {
    let _clip = Clipboard::new_attempts(10)?;
    get(format)
}

#[inline(always)]
///Set data onto clipboard.
pub fn set<R, T: Setter<R>>(format: T, data: R) -> SysResult<()> {
    format.write_clipboard(&data)
}

#[inline(always)]
///Shortcut to set data onto clipboard.
///
///It opens clipboard and attempts to set data.
pub fn set_clipboard<R, T: Setter<R>>(format: T, data: R) -> SysResult<()> {
    let _clip = Clipboard::new_attempts(10)?;
    set(format, data)
}

///Shortcut to retrieve string from clipboard.
///
///It opens clipboard and gets string, if possible.
#[inline(always)]
pub fn get_clipboard_string() -> SysResult<alloc::string::String> {
    get_clipboard(Unicode)
}

///Shortcut to set string onto clipboard.
///
///It opens clipboard and attempts to set string.
#[inline(always)]
pub fn set_clipboard_string(data: &str) -> SysResult<()> {
    set_clipboard(Unicode, data)
}



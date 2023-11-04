use crate::SysResult;
use core_utils::ffi::types::*;
use crate::sys::*;
use error_code::ErrorCode;
use core::{slice, mem, ptr, cmp};
use core::num::{NonZeroUsize, NonZeroU32};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline]
/// Opens clipboard.
/// 打开剪贴板以供检查，并阻止其他应用程序修改剪贴板内容。
/// 在任何时候，只有一个程序可以打开剪贴簿。
/// 调用OpenClipboard的作用是当一个程序使用剪贴簿时，防止剪贴簿的内容发生变化。
/// OpenClipboard传回BOOL值，它说明是否已经成功地打开了剪贴簿。
/// 如果另一个应用程序没有关闭剪贴簿，那么它就不能被打开。
/// 如果每个程序在响应使用者的命令时都尽快地、遵守规范地打开然后关闭剪贴簿，那么将永远不会遇到不能打开剪贴簿的问题。
pub fn open_for(owner: HWND) -> SysResult<()> {
    match unsafe { OpenClipboard(owner) } {
        0 => Err(ErrorCode::last_system()), // 如果函数失败，则返回值为零
        _ => Ok(()),    // 如果该函数成功，则返回值为非零值。
    }
}

///Opens clipboard.
pub fn open() -> SysResult<()> {
    // 如果此参数为 NULL，则打开的剪贴板与当前任务相关联
    open_for(ptr::null_mut())
}

#[inline]
///Closes clipboard.
pub fn close() -> SysResult<()> {
    match unsafe { CloseClipboard() } {
        0 => Err(ErrorCode::last_system()),
        _ => Ok(()),
    }
}

#[inline]
///Empties clipboard.
pub fn empty() -> SysResult<()> {
    match unsafe { EmptyClipboard() } {
        0 => Err(ErrorCode::last_system()),
        _ => Ok(()),
    }
}

#[inline]
///Retrieves clipboard sequence number.
pub fn seq_num() -> Option<NonZeroU32> {
    unsafe { NonZeroU32::new(GetClipboardSequenceNumber()) }
}

#[inline]
///Retrieves size of clipboard data for specified format.
/// 
///# Unsafety:
///
///In some cases, clipboard content might be so invalid that it crashes on `GlobalSize` (e.g.
///Bitmap)
///
///Due to that function is marked as unsafe
pub unsafe fn size_unsafe(format: u32) -> Option<NonZeroUsize> {
    let clipboard_data = GetClipboardData(format);

    match clipboard_data.is_null() {
        false => NonZeroUsize::new(GlobalSize(clipboard_data) as usize),
        true => None,
    }
}

#[inline]
///Retrieves size of clipboard data for specified format.
pub fn size(format: u32) -> Option<NonZeroUsize> {
    let clipboard_data = unsafe {GetClipboardData(format)};

    if clipboard_data.is_null() {
        return None
    }

    unsafe {
        if GlobalLock(clipboard_data).is_null() {
            return None;
        }

        let result = NonZeroUsize::new(GlobalSize(clipboard_data) as usize);

        GlobalUnlock(clipboard_data);

        result
    }
}

#[inline(always)]
///Retrieves raw pointer to clipboard data.
pub fn get_clipboard_data(format: c_uint) -> SysResult<ptr::NonNull<c_void>> {
    let ptr = unsafe {
        GetClipboardData(format)
    };
    match ptr::NonNull::new(ptr) {
        Some(ptr) => Ok(ptr),
        None => Err(ErrorCode::last_system()),
    }
}

#[inline(always)]
///Determines whenever provided clipboard format is available on clipboard or not.
pub fn is_format_avail(format: c_uint) -> bool {
    unsafe { IsClipboardFormatAvailable(format) != 0 }
}

#[inline]
///Retrieves number of currently available formats on clipboard.
///
///Returns `None` if `CountClipboardFormats` failed.
pub fn count_formats() -> Option<usize> {
    let result = unsafe { CountClipboardFormats() };

    if result == 0 {
        // 如果函数失败，则返回值为零。 要获得更多的错误信息，请调用 GetLastError。
        if ErrorCode::last_system().raw_code() != 0 {
            return None
        }
    }

    // 如果函数成功，则返回值是剪贴板上当前不同数据格式的数目。
    Some(result as usize)
}


#[inline(always)]
///Retrieves the window handle of the current owner of the clipboard.
///
///Returns `None` if clipboard is not owned.
pub fn get_owner() -> Option<ptr::NonNull::<c_void>> {
    ptr::NonNull::new(unsafe {
        GetClipboardOwner()
    })
}

#[inline(always)]
fn free_dc(data: HDC) {
    unsafe {
        ReleaseDC(ptr::null_mut(), data);
    }
}

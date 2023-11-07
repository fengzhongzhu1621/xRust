use crate::ffi::types::*;
use crate::ffi::win::SysResult;
use crate::ffi::win::sys::*;
use crate::ffi::win::CP_UTF8;
use crate::ffi::buffer::Buffer;
use crate::ffi::win::utils::*;

use error_code::ErrorCode;
use core::{ptr, mem, slice};
use core::num::NonZeroU32;
use core::num::NonZeroUsize;

///A handle to a bitmap (HBITMAP).
pub const CF_BITMAP: c_uint = 2;
///A memory object containing a <b>BITMAPINFO</b> structure followed by the bitmap bits.
pub const CF_DIB: c_uint = 8;
///A memory object containing a <b>BITMAPV5HEADER</b> structure followed by the bitmap color space
///information and the bitmap bits.
pub const CF_DIBV5: c_uint = 17;
///Software Arts' Data Interchange Format.
pub const CF_DIF: c_uint = 5;
///Bitmap display format associated with a private format. The hMem parameter must be a handle to
///data that can be displayed in bitmap format in lieu of the privately formatted data.
pub const CF_DSPBITMAP: c_uint = 0x0082;
///Enhanced metafile display format associated with a private format. The *hMem* parameter must be a
///handle to data that can be displayed in enhanced metafile format in lieu of the privately
///formatted data.
pub const CF_DSPENHMETAFILE: c_uint = 0x008E;
///Metafile-picture display format associated with a private format. The hMem parameter must be a
///handle to data that can be displayed in metafile-picture format in lieu of the privately
///formatted data.
pub const CF_DSPMETAFILEPICT: c_uint = 0x0083;
///Text display format associated with a private format. The *hMem* parameter must be a handle to
///data that can be displayed in text format in lieu of the privately formatted data.
pub const CF_DSPTEXT: c_uint = 0x0081;
///A handle to an enhanced metafile (<b>HENHMETAFILE</b>).
pub const CF_ENHMETAFILE: c_uint = 14;
///Start of a range of integer values for application-defined GDI object clipboard formats.
pub const CF_GDIOBJFIRST: c_uint = 0x0300;
///End of a range of integer values for application-defined GDI object clipboard formats.
pub const CF_GDIOBJLAST: c_uint = 0x03FF;
///A handle to type <b>HDROP</b> that identifies a list of files.
pub const CF_HDROP: c_uint = 15;
///The data is a handle to the locale identifier associated with text in the clipboard.
///
///For details see [Standart Clipboard Formats](https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx)
pub const CF_LOCALE: c_uint = 16;
///Handle to a metafile picture format as defined by the <b>METAFILEPICT</b> structure.
pub const CF_METAFILEPICT: c_uint = 3;
///Text format containing characters in the OEM character set.
pub const CF_OEMTEXT: c_uint = 7;
///Owner-display format.
///
///For details see [Standart Clipboard Formats](https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx)
pub const CF_OWNERDISPLAY: c_uint = 0x0080;
///Handle to a color palette.
///
///For details see [Standart Clipboard Formats](https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx)
pub const CF_PALETTE: c_uint = 9;
///Data for the pen extensions to the Microsoft Windows for Pen Computing.
pub const CF_PENDATA: c_uint = 10;
///Start of a range of integer values for private clipboard formats.
pub const CF_PRIVATEFIRST: c_uint = 0x0200;
///End of a range of integer values for private clipboard formats.
pub const CF_PRIVATELAST: c_uint = 0x02FF;
///Represents audio data more complex than can be represented in a ```CF_WAVE``` standard wave format.
pub const CF_RIFF: c_uint = 11;
///Microsoft Symbolic Link (SYLK) format.
pub const CF_SYLK: c_uint = 4;
///ANSI text format.
pub const CF_TEXT: c_uint = 1;
///Tagged-image file format.
pub const CF_TIFF: c_uint = 6;
///UTF16 text format.
/// Unicode 文本格式。 每行以回车符/换行符 (CR-LF) 组合结束。 null 字符表示数据结束。
pub const CF_UNICODETEXT: c_uint = 13;
///Represents audio data in one of the standard wave formats.
pub const CF_WAVE: c_uint = 12;

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

///Enumerator over available clipboard formats.
///
///# Pre-conditions:
///
///* [open()](fn.open.html) has been called.
pub struct EnumFormats {
    idx: u32
}

impl EnumFormats {
    /// Constructs enumerator over all available formats.
    pub fn new() -> EnumFormats {
        EnumFormats { idx: 0 }
    }

    /// Constructs enumerator that starts from format.
    pub fn from(format: u32) -> EnumFormats {
        EnumFormats { idx: format }
    }

    /// Resets enumerator to list all available formats.
    pub fn reset(&mut self) -> &EnumFormats {
        self.idx = 0;
        self
    }
}

impl Iterator for EnumFormats {
    type Item = u32;

    /// Returns next format on clipboard.
    ///
    /// In case of failure (e.g. clipboard is closed) returns `None`.
    fn next(&mut self) -> Option<u32> {
        self.idx = unsafe { EnumClipboardFormats(self.idx) };

        if self.idx == 0 {
            None
        } else {
            Some(self.idx)
        }
    }

    /// Relies on `count_formats` so it is only reliable
    /// when hinting size for enumeration of all formats.
    ///
    /// Doesn't require opened clipboard.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, count_formats())
    }
}


macro_rules! match_format_name_big {
    ( $name:expr, $( $f:ident ),* ) => {
        match $name {
            $( $f => Some(stringify!($f).to_owned()),)*
            CF_GDIOBJFIRST ..= CF_GDIOBJLAST => Some(format!("CF_GDIOBJ{}", $name - CF_GDIOBJFIRST)),
            CF_PRIVATEFIRST ..= CF_PRIVATELAST => Some(format!("CF_PRIVATE{}", $name - CF_PRIVATEFIRST)),
            _ => {
                let mut format_buff = [0u16; 256];
                unsafe {
                    let buff_p = format_buff.as_mut_ptr() as *mut u16;
                    // 从剪贴板检索指定注册格式的名称, 将名称复制到指定的缓冲区。
                    match GetClipboardFormatNameW($name, buff_p, format_buff.len() as c_int) {
                        0 => None,
                        size => Some(String::from_utf16_lossy(&format_buff[..size as usize])),
                    }
                }
            }
        }
    }
}

macro_rules! match_format_name {
    ( $name:expr => $out:ident, $( $f:ident ),* ) => {
        use core::fmt::Write;

        match $name {
            $( $f => {
                let _ = $out.push_str(stringify!($f));
            },)*
            CF_GDIOBJFIRST ..= CF_GDIOBJLAST => {
                let _ = write!($out, "CF_GDIOBJ{}", $name - CF_GDIOBJFIRST);
            },
            CF_PRIVATEFIRST ..= CF_PRIVATELAST => {
                let _ = write!($out, "CF_PRIVATE{}", $name - CF_PRIVATEFIRST);
            },
            _ => {
                let mut format_buff = [0u16; 256];
                unsafe {
                    let buff_p = format_buff.as_mut_ptr() as *mut u16;
                    match GetClipboardFormatNameW($name, buff_p, format_buff.len() as c_int) {
                        0 => return None,
                        len => match WideCharToMultiByte(CP_UTF8, 0, format_buff.as_ptr(), len, $out.as_mut_ptr() as *mut i8, $out.remaining() as i32, ptr::null(), ptr::null_mut()) {
                            0 => return None,
                            len => $out.set_len(len as _),
                        }
                    }
                }
            }
        }

        return $out.as_str()
    }
}

///Returns format name based on it's code.
pub fn format_name(format: u32, mut out: Buffer<'_>) -> Option<&'_ str> {
    match_format_name!(format => out,
                       CF_BITMAP,
                       CF_DIB,
                       CF_DIBV5,
                       CF_DIF,
                       CF_DSPBITMAP,
                       CF_DSPENHMETAFILE,
                       CF_DSPMETAFILEPICT,
                       CF_DSPTEXT,
                       CF_ENHMETAFILE,
                       CF_HDROP,
                       CF_LOCALE,
                       CF_METAFILEPICT,
                       CF_OEMTEXT,
                       CF_OWNERDISPLAY,
                       CF_PALETTE,
                       CF_PENDATA,
                       CF_RIFF,
                       CF_SYLK,
                       CF_TEXT,
                       CF_WAVE,
                       CF_TIFF,
                       CF_UNICODETEXT);
}

///Returns format name based on it's code (allocating variant suitable for big names)
pub fn format_name_big(format: u32) -> Option<String> {
    match_format_name_big!(format,
                           CF_BITMAP,
                           CF_DIB,
                           CF_DIBV5,
                           CF_DIF,
                           CF_DSPBITMAP,
                           CF_DSPENHMETAFILE,
                           CF_DSPMETAFILEPICT,
                           CF_DSPTEXT,
                           CF_ENHMETAFILE,
                           CF_HDROP,
                           CF_LOCALE,
                           CF_METAFILEPICT,
                           CF_OEMTEXT,
                           CF_OWNERDISPLAY,
                           CF_PALETTE,
                           CF_PENDATA,
                           CF_RIFF,
                           CF_SYLK,
                           CF_TEXT,
                           CF_WAVE,
                           CF_TIFF,
                           CF_UNICODETEXT)
}


#[inline]
///Registers a new clipboard format with specified name as C wide string (meaning it must have null
///char at the end).
///
///# Returns:
///
///Newly registered format identifier, if successful.
///
///# Note:
///
///- Custom format identifier is in range `0xC000...0xFFFF`.
///- Function fails if input is not null terminated string.
pub unsafe fn register_raw_format(name: &[u16]) -> Option<NonZeroU32> {
    if name.is_empty() || name[name.len()-1] != b'\0' as u16 {
        return unlikely_empty_size_result()
    }
    NonZeroU32::new(RegisterClipboardFormatW(name.as_ptr()) )
}

///Registers a new clipboard format with specified name.
///
///# Returns:
///
///Newly registered format identifier, if successful.
///
///# Note:
///
///Custom format identifier is in range `0xC000...0xFFFF`.
pub fn register_format(name: &str) -> Option<NonZeroU32> {
    let size = unsafe {
        MultiByteToWideChar(CP_UTF8, 0, name.as_ptr() as *const _, name.len() as c_int, ptr::null_mut(), 0)
    };

    if size == 0 {
        return unlikely_empty_size_result()
    }

    if size > 52 {
        let mut buffer = alloc::vec::Vec::with_capacity(size as usize);
        let size = unsafe {
            MultiByteToWideChar(CP_UTF8, 0, name.as_ptr() as *const _, name.len() as c_int, buffer.as_mut_ptr(), size)
        };
        unsafe {
            buffer.set_len(size as usize);
            buffer.push(0);
            register_raw_format(&buffer)
        }
    } else {
        let mut buffer = mem::MaybeUninit::<[u16; 52]>::uninit();
        let size = unsafe {
            MultiByteToWideChar(CP_UTF8, 0, name.as_ptr() as *const _, name.len() as c_int, buffer.as_mut_ptr() as *mut u16, 51)
        };
        unsafe {
            ptr::write((buffer.as_mut_ptr() as *mut u16).offset(size as isize), 0);
            register_raw_format(slice::from_raw_parts(buffer.as_ptr() as *const u16, size as usize + 1))
        }
    }
}

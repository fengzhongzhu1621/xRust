//! WINAPI related types

#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use core::ffi::c_void;
pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;
pub type __int8 = i8;
pub type __uint8 = u8;
pub type __int16 = i16;
pub type __uint16 = u16;
pub type __int32 = i32;
pub type __uint32 = u32;
pub type __int64 = i64;
pub type __uint64 = u64;
pub type wchar_t = u16;
pub type HANDLE = *mut c_void;  // 等于 C 的 void*。
pub type HGLOBAL = HANDLE;
pub type BOOL = c_int;
pub type ULONG_PTR = usize;
pub type SIZE_T = ULONG_PTR;
pub type HWND = HANDLE;
pub type WORD = c_ushort;
pub type DWORD = c_ulong;
pub type LONG = c_long;
pub type LPVOID = *mut c_void;
pub type HDC = *mut c_void;
pub type HDROP = *mut c_void;
pub type HBITMAP = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINT {
    pub x: c_long,
    pub y: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BITMAPINFOHEADER {
    pub biSize: DWORD,
    pub biWidth: LONG,
    pub biHeight: LONG,
    pub biPlanes: WORD,
    pub biBitCount: WORD,
    pub biCompression: DWORD,
    pub biSizeImage: DWORD,
    pub biXPelsPerMeter: LONG,
    pub biYPelsPerMeter: LONG,
    pub biClrUsed: DWORD,
    pub biClrImportant: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RGBQUAD {
    pub rgbBlue: c_uchar,
    pub rgbGreen: c_uchar,
    pub rgbRed: c_uchar,
    pub rgbReserved: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BITMAP {
    pub bmType: LONG,
    pub bmWidth: LONG,
    pub bmHeight: LONG,
    pub bmWidthBytes: LONG,
    pub bmPlanes: WORD,
    pub bmBitsPixel: WORD,
    pub bmBits: LPVOID,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct BITMAPFILEHEADER {
    pub bfType: WORD,
    pub bfSize: DWORD,
    pub bfReserved1: WORD,
    pub bfReserved2: WORD,
    pub bfOffBits: DWORD,
}


/// 内存分配属性：如果指定零，则默认值为 GMEM_FIXED。 此参数可以是以下一个或多个值，但专门指出的不兼容组合除外。
/// GHND 0x0042 将 GMEM_MOVEABLE 和GMEM_ZEROINIT组合在 一起。
/// GMEM_FIXED 0x0000 分配固定内存。 返回值为指针。
/// GMEM_MOVEABLE 0x0002 分配可移动内存。 内存块永远不会在物理内存中移动，但它们可以在默认堆中移动。返回值是内存对象的句柄。 若要将句柄转换为指针，请使用 GlobalLock 函数。此值不能与 GMEM_FIXED 组合使用。
/// GMEM_ZEROINIT 0x0040 将内存内容初始化为零。
/// GPTR 0x0040 将 GMEM_FIXED 和GMEM_ZEROINIT组合在 一起。
pub const GHND: c_uint = 0x42;

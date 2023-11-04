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
pub type HANDLE = *mut c_void;
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

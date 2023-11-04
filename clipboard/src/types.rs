//! WINAPI related types

#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use core_utils::ffi::types::*;


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

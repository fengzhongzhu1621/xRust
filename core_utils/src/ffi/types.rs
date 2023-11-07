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
    pub biSize: DWORD, // 指定这个结构的长度，为40。 
    pub biWidth: LONG, // 指定图象的宽度，单位是象素。
    pub biHeight: LONG, // 指定图象的高度，单位是象素。
    pub biPlanes: WORD, // 必须是1
    pub biBitCount: WORD, // 指定表示颜色时要用到的位数，常用的值为1(黑白二色图), 4(16色图), 8(256色), 24(真彩色图)(新的.bmp格式支持32位色)。
    pub biCompression: DWORD, // 指定位图是否压缩，有效的值为BI_RGB，BI_RLE8，BI_RLE4，BI_BITFIELDS(都是一些Windows定义好的常量)。
    pub biSizeImage: DWORD, // 指定实际的位图数据占用的字节数，其实也可以从以下的公式中计算出来： biSizeImage=biWidth × biHeight 
    pub biXPelsPerMeter: LONG, // 指定目标设备的水平分辨率，单位是每米的象素个数
    pub biYPelsPerMeter: LONG, // 指定目标设备的垂直分辨率，单位同上。 
    pub biClrUsed: DWORD,  // 指定本图象实际用到的颜色数，如果该值为零，则用到的颜色数为2biBitCount。
    pub biClrImportant: DWORD,  // 指定本图象中重要的颜色数，如果该值为零，则认为所有的颜色都是重要的。
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
// 定义了DIB(设备无关位图)的大小和颜色信息。
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER, // 包含了关于大小尺寸和颜色格式信息
    pub bmiColors: [RGBQUAD; 1], // 1、RGBQUAD数组，每个项组成了颜色表 2、16位无符号整型数组，指定了当前以实现的逻辑调色板的索引
}

#[repr(C)]
#[derive(Copy, Clone)]
// 定义了逻辑位图的高度、宽度、颜色格式和位值。
pub struct BITMAP {
    pub bmType: LONG, // 指定了位图的类型，对于逻辑位图该参数必须为0
    pub bmWidth: LONG, // 指定了位图的宽度(以字节为单位)，必须大于0
    pub bmHeight: LONG, // 指定了位图的高度(以字节为单位)，必须大于0
    pub bmWidthBytes: LONG, // 每行字节数，4位对齐
    pub bmPlanes: WORD, // 指定了颜色平面数
    pub bmBitsPixel: WORD, // 指定了每个像素的位数，比如RGB每个像素占3个字节，即24位
    pub bmBits: LPVOID, // 指向位图数据内存的地址
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

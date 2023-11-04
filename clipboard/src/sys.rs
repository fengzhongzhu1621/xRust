use super::types::*;

// https://learn.microsoft.com/zh-cn/windows/win32/api/_base/

extern "system" {
    // 锁定内存块，该函数接受一个内存句柄作为参数，然后返回一个指向被锁定的内存块的指针, 可以用该指针来读写内存。
    pub fn GlobalLock(hMem: HGLOBAL) -> LPVOID;
    // 解锁先前被锁定的内存，该函数使得指向内存块的指针无效。
    pub fn GlobalUnlock(hmem: HGLOBAL) -> BOOL;
    // 释放内存块。必须传给该函数一个内存句柄
    pub fn GlobalFree(hmem: HGLOBAL) -> HGLOBAL;
    // 分配的实际内存大小可以大于请求的大小。 获取实际分配的字节数
    pub fn GlobalSize(hMem: HGLOBAL) -> SIZE_T;
    // 从堆中分配指定的字节数。返回一个指针，调用进程可以立即使用该指针来访问内存。
    pub fn GlobalAlloc(uflags: c_uint, dwbytes: SIZE_T) -> HGLOBAL;
    // 使计算机程序（进程，任务或线程）进入休眠，使其在一段时间内处于非活动状态。当函数设定的计时器到期，或者接收到信号、程序发生中断都会导致程序继续执行。
    // 需要一个以毫秒为单位的参数代表程序挂起时长
    pub fn Sleep(dwMilliseconds: DWORD);

    // 映射一个unicode字符串到一个多字节字符串
    // 将 UTF-16 (宽字符) 字符串映射到新的字符串。 新字符串不一定来自多字节字符集。
    // 从 UTF-16 转换为非 Unicode 编码的数据可能会丢失数据，因为代码页可能无法表示特定 Unicode 数据中使用的每个字符。
    // 
    // Note: 谨慎 错误地使用 WideCharToMultiByte 函数可能会危及应用程序的安全性。 
    // 调用此函数很容易导致缓冲区溢出，因为 lpWideCharStr 指示的输入缓冲区的大小等于 Unicode 字符串中的字符数，
    // 而 lpMultiByteStr 指示的输出缓冲区的大小等于字节数。 若要避免缓冲区溢出，应用程序必须指定适合缓冲区接收的数据类型的缓冲区大小。
    // 
    // int WideCharToMultiByte(
    // [in]            UINT                               CodePage,
    // [in]            DWORD                              dwFlags,
    // [in]            _In_NLS_string_(cchWideChar)LPCWCH lpWideCharStr,
    // [in]            int                                cchWideChar,
    // [out, optional] LPSTR                              lpMultiByteStr,
    // [in]            int                                cbMultiByte,
    // [in, optional]  LPCCH                              lpDefaultChar,
    // [out, optional] LPBOOL                             lpUsedDefaultChar
    // );
    pub fn WideCharToMultiByte(page: c_uint, flags: c_ulong, wide_str: *const u16, wide_str_len: c_int,
                               multi_str: *mut i8, multi_str_len: c_int,
                               default_char: *const i8, used_default_char: *mut bool) -> c_int;
    // 将字符串映射到 UTF-16 (宽字符) 字符串。 字符串不一定来自多字节字符集。
    //
    // Note: 谨慎 错误地使用 MultiByteToWideChar 函数可能会危及应用程序的安全性。 
    // 调用此函数很容易导致缓冲区溢出，因为 lpMultiByteStr 指示的输入缓冲区的大小等于字符串中的字节数，
    // 而 lpWideCharStr 指示的输出缓冲区的大小等于字符数。 
    // 若要避免缓冲区溢出，应用程序必须指定适合缓冲区接收的数据类型的缓冲区大小。
    //
    // int MultiByteToWideChar(
    //     [in]            UINT                              CodePage,
    //     [in]            DWORD                             dwFlags,
    //     [in]            _In_NLS_string_(cbMultiByte)LPCCH lpMultiByteStr,
    //     [in]            int                               cbMultiByte,
    //     [out, optional] LPWSTR                            lpWideCharStr,
    //     [in]            int                               cchWideChar
    //   );
    pub fn MultiByteToWideChar(CodePage: c_uint, dwFlags: DWORD, lpMultiByteStr: *const u8, cbMultiByte: c_int, lpWideCharStr: *mut u16, cchWideChar: c_int) -> c_int;
}

extern "system" {
    // 释放设备上下文，释放它供其他应用程序使用。
    //  ReleaseDC 函数的效果取决于 DC 的类型。 它仅释放公用 DC 和窗口 DC。 它对类或专用 DC 没有影响。
    // 当使用公共DC绘制后，必须调用ReleaseDC函数来释放DC。
    // ReleaseDC必须和GetDC同属于同一个线程。DC的数量仅局限于可用的内存大小。
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
    // 检索指定窗口的工作区或整个屏幕的设备上下文 (DC) 的句柄。 
    // 可以在后续 GDI 函数中使用返回的句柄在 DC 中绘制。 
    // 设备上下文是一种不透明的数据结构，其值由 GDI 在内部使用。
    pub fn GetDC(hWnd: HWND) -> HDC;

    // 打开剪贴板以供检查，并阻止其他应用程序修改剪贴板内容。
    // 如果另一个窗口打开了剪贴板，OpenClipboard 将失败。
    // 每次成功调用 OpenClipboard 后，应用程序都应调用 CloseClipboard 函数。
    // 除非调用 EmptyClipboard 函数，否则由 hWndNewOwner 参数标识的窗口不会成为剪贴板所有者。
    pub fn OpenClipboard(hWnd: HWND) -> BOOL;
    // 当窗口完成检查或更改剪贴板后，通过调用 CloseClipboard 关闭剪贴板。 这使其他窗口能够访问剪贴板。
    // 调用 CloseClipboard 后，不要将对象放在剪贴板上。
    pub fn CloseClipboard() -> BOOL;
    // 清空剪贴板并释放剪贴板中数据的句柄。 然后， 函数将剪贴板的所有权分配给当前已打开剪贴板的窗口。
    // 在调用 EmptyClipboard 之前，应用程序必须使用 OpenClipboard 函数打开剪贴板。 
    // 如果应用程序在打开剪贴板时指定 NULL 窗口句柄， EmptyClipboard 将成功，但将剪贴板所有者设置为 NULL。 
    // 请注意，这会导致 SetClipboardData 失败。
    pub fn EmptyClipboard() -> BOOL;
    // 检索当前窗口工作站的剪贴板序列号。
    // 系统为每个窗口工作站保留剪贴板的序列号。 每当剪贴板的内容更改或剪贴板被清空时，此数字将递增。 
    // 可以跟踪此值以确定剪贴板内容是否已更改并优化创建 DataObject。 
    // 如果剪贴板呈现延迟，则在呈现更改之前，序列号不会递增。
    pub fn GetClipboardSequenceNumber() -> DWORD;
    // 确定剪贴板是否包含指定格式的数据。
    // https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/standard-clipboard-formats
    pub fn IsClipboardFormatAvailable(format: c_uint) -> BOOL;
    // 检索剪贴板上当前不同数据格式的数量。
    pub fn CountClipboardFormats() -> c_int;
    // 枚举剪贴板上当前可用的数据格式。
    // 剪贴板数据格式存储在有序列表中。 若要执行剪贴板数据格式的枚举，需要对 EnumClipboardFormats 函数进行一系列调用。 
    // 对于每次调用， format 参数指定可用的剪贴板格式，函数返回下一个可用的剪贴板格式。
    pub fn EnumClipboardFormats(format: c_uint) -> c_uint;
    // 从剪贴板检索指定注册格式的名称。 函数将名称复制到指定的缓冲区。
    pub fn GetClipboardFormatNameW(format: c_uint, lpszFormatName: *mut u16, cchMaxCount: c_int) -> c_int;
    // 注册新的剪贴板格式。 然后，可以将此格式用作有效的剪贴板格式。
    // 如果已存在具有指定名称的已注册格式，则不会注册新格式，并且返回值标识现有格式。 
    // 这使多个应用程序能够使用相同的注册剪贴板格式复制和粘贴数据。 请注意，格式名称比较不区分大小写。
    pub fn RegisterClipboardFormatW(lpszFormat: *const u16) -> c_uint;
    // 从剪贴板中检索指定格式的数据。 剪贴板之前必须已打开。
    // Note: 剪贴板数据不受信任。 在应用程序中使用数据之前，请仔细分析数据。
    // 剪贴板控制 GetClipboardData 函数返回的句柄，而不是应用程序。 应用程序应立即复制数据。 
    // 应用程序不得释放句柄，也不能将其保持锁定状态。
    // 在调用 EmptyClipboard 或 CloseClipboard 函数后，或者在使用相同的剪贴板格式调用 SetClipboardData 函数之后，应用程序不得使用句柄。
    // 当应用程序调用 GetClipboardData 函数时，系统会在某些剪贴板格式之间执行隐式数据格式转换。 
    // 例如，如果 CF_OEMTEXT 格式位于剪贴板上，则窗口可以检索 CF_TEXT 格式的数据。 剪贴板上的格式将按需转换为请求的格式。
    // https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/clipboard-formats
    pub fn GetClipboardData(uFormat: c_uint) -> HANDLE;
    // 将数据以指定的剪贴板格式放置在剪贴板上。 窗口必须是当前剪贴板所有者，并且应用程序必须已调用 OpenClipboard 函数。
    // (响应WM_RENDERFORMAT消息时，剪贴板所有者不得在调用 SetClipboardData.)
    pub fn SetClipboardData(uFormat: c_uint, hMem: HANDLE) -> HANDLE;
    // 检索剪贴板当前所有者的窗口句柄。
    pub fn GetClipboardOwner() -> HWND;

    // 检索由成功拖放操作生成的已删除文件的名称。
    pub fn DragQueryFileW(hDrop: HDROP, iFile: c_uint, lpszFile: *mut u16, cch: c_uint) -> c_uint;

}

extern "system" {
    // 从 DIB 创建兼容的位图 (DDB) ，并选择性地设置位图位。
    pub fn CreateDIBitmap(hdc: HDC, pbmih: *const BITMAPINFOHEADER, flInit: DWORD, pjBits: *const c_void, pbmi: *const BITMAPINFO, iUsage: c_uint) -> HBITMAP;
    // 检索指定兼容位图的位，并使用指定格式将其作为 DIB 复制到缓冲区中。
    pub fn GetDIBits(hdc: HDC, hbm: HBITMAP, start: c_uint, cLines: c_uint, lpvBits: *mut c_void, lpbmi: *mut BITMAPINFO, usage: c_uint) -> c_int;
    // 检索指定图形对象的信息。
    pub fn GetObjectW(h: HANDLE, c: c_int, pv: *mut c_void) -> c_int;
}

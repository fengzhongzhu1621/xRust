use core::{slice, mem, ptr, cmp};
use core_utils::ffi::win::*;
use core_utils::ffi::types::*;


///Copies raw bytes from clipboard with specified `format`
///
///Returns number of copied bytes on success, otherwise 0.
///
///It is safe to pass uninit memory
pub fn get(format: u32, out: &mut [u8]) -> SysResult<usize> {
    let size = out.len();
    if size == 0 {
        return Ok(unlikely_empty_size_result());
    }
    // 转换为  *mut u8 类型
    let out_ptr = out.as_mut_ptr();

    // 返回指向剪贴板数据的指针
    let ptr = RawMem::from_borrowed(clipboard::get_clipboard_data(format)?);

    let result = unsafe {
        // 加锁，返回一个指向被锁定的内存块的指针
        let (data_ptr, _lock) = ptr.lock()?;
        // 返回内存块的大小，不能超过缓存的大小
        let data_size = cmp::min(GlobalSize(ptr.get()) as usize, size);
        // 拷贝剪贴板的内容到缓存中
        ptr::copy_nonoverlapping(data_ptr.as_ptr() as *const u8, out_ptr, data_size);

        // 返回拷贝内容的大小
        data_size
    };

    Ok(result)
}

///Copies raw bytes from clipboard with specified `format`, appending to `out` buffer.
///
///Returns number of copied bytes on success, otherwise 0.
pub fn get_vec(format: u32, out: &mut alloc::vec::Vec<u8>) -> SysResult<usize> {
    let ptr = RawMem::from_borrowed(clipboard::get_clipboard_data(format)?);

    let result = unsafe {
        // 加锁，返回一个指向被锁定的内存块的指针
        let (data_ptr, _lock) = ptr.lock()?;
        // 返回剪贴板内存的大小
        let data_size = GlobalSize(ptr.get()) as usize;

        // 追加缓存的大小，如果新容量超过 isize::MAX 字节，则会出现Panics。
        out.reserve(data_size as usize);
        let storage_cursor = out.len();

        // 移动目标指针
        let storage_ptr = out.as_mut_ptr().add(storage_cursor) as *mut _;

        // 拷贝剪贴板内容
        ptr::copy_nonoverlapping(data_ptr.as_ptr() as *const u8, storage_ptr, data_size);

        // 重新设置数组的长度
        out.set_len(storage_cursor + data_size as usize);

        data_size
    };

    Ok(result)
}

///Copies raw bytes from clipboard with specified `format`, appending to `out` buffer.
///
///Returns number of copied bytes on success, otherwise 0.
pub fn get_string(out: &mut alloc::vec::Vec<u8>) -> SysResult<usize> {
    //获取 Unicode 文本格式的剪贴板数据。 每行以回车符/换行符 (CR-LF) 组合结束。 null 字符表示数据结束。
    let ptr = RawMem::from_borrowed(clipboard::get_clipboard_data(clipboard::CF_UNICODETEXT)?);

    let result = unsafe {
        let (data_ptr, _lock) = ptr.lock()?;
        // 获得Unicode字符的数量
        let data_size = GlobalSize(ptr.get()) as usize / mem::size_of::<u16>();
        // 获得转换为utf8的字节数
        let storage_req_size = WideCharToMultiByte(CP_UTF8, 0, data_ptr.as_ptr() as _, data_size as _, 
        ptr::null_mut(), 0, ptr::null(), ptr::null_mut());
        if storage_req_size == 0 {
            return Err(ErrorCode::last_system());
        }

        // 映射一个unicode字符串到一个多字节字符串
        // CP_UTF8: UTF-8。 设置此值后， 必须将 lpDefaultChar 和 lpUsedDefaultChar 设置为 NULL。
        let storage_cursor = out.len();
        out.reserve(storage_req_size as usize);
        let storage_ptr = out.as_mut_ptr().add(storage_cursor) as *mut _;
        WideCharToMultiByte(CP_UTF8, 0, data_ptr.as_ptr() as _, data_size as _, storage_ptr, storage_req_size, ptr::null(), ptr::null_mut());
        out.set_len(storage_cursor + storage_req_size as usize);

        //It seems WinAPI always supposed to have at the end null char.
        //But just to be safe let's check for it and only then remove.
        if let Some(null_idx) = out.iter().skip(storage_cursor).position(|b| *b == b'\0') {
            out.set_len(storage_cursor + null_idx);
        }

        out.len() - storage_cursor
    };

    Ok(result)
}

#[cfg(feature = "std")]
///Retrieves file list from clipboard, appending each element to the provided storage.
///
///Returns number of appended file names.
pub fn get_file_list_path(out: &mut alloc::vec::Vec<std::path::PathBuf>) -> SysResult<usize> {
    use std::os::windows::ffi::OsStringExt;

    let clipboard_data = RawMem::from_borrowed(clipboard::get_clipboard_data(clipboard::CF_HDROP)?);

    let (_data_ptr, _lock) = clipboard_data.lock()?;

    // 用于检索拖放操作所被拖放文件的名称。这里也可以用于复制到剪切板内的文件，返回文件的数量
    let num_files = unsafe { DragQueryFileW(clipboard_data.get() as _, u32::MAX, ptr::null_mut(), 0) };
    out.reserve(num_files as usize);

    let mut buffer = alloc::vec::Vec::new();

    for idx in 0..num_files {
        // 返回为文件名的长度，不包括终止 null 字符
        let required_size_no_null = unsafe { DragQueryFileW(clipboard_data.get() as _, idx, ptr::null_mut(), 0) };
        if required_size_no_null == 0 {
            return Err(ErrorCode::last_system());
        }

        let required_size = required_size_no_null + 1;
        buffer.reserve(required_size as usize);

        // 获取文件名到buffer中
        if unsafe { DragQueryFileW(clipboard_data.get() as _, idx, buffer.as_mut_ptr(), required_size) == 0 } {
            return Err(ErrorCode::last_system());
        }

        unsafe {
            buffer.set_len(required_size_no_null as usize);
        }
        //This fucking abomination of API requires double allocation,
        //just because no one had brain for to provide API for creation OsString out of owned
        //Vec<16>
        // 从可能的 ill-formed UTF-16 16 位代码单元切片创建 OsString。并转换为PathBuf类型
        out.push(std::ffi::OsString::from_wide(&buffer).into())
    }

    // 返回文件的数量
    Ok(num_files as usize)
}

///Retrieves file list from clipboard, appending each element to the provided storage.
///
///Returns number of appended file names.
pub fn get_file_list(out: &mut alloc::vec::Vec<alloc::string::String>) -> SysResult<usize> {
    let clipboard_data = RawMem::from_borrowed(clipboard::get_clipboard_data(clipboard::CF_HDROP)?);

    let (_data_ptr, _lock) = clipboard_data.lock()?;

    let num_files = unsafe { DragQueryFileW(clipboard_data.get() as _, u32::MAX, ptr::null_mut(), 0) };
    out.reserve(num_files as usize);

    let mut buffer = alloc::vec::Vec::new();

    for idx in 0..num_files {
        let required_size_no_null = unsafe { DragQueryFileW(clipboard_data.get() as _, idx, ptr::null_mut(), 0) };
        if required_size_no_null == 0 {
            return Err(ErrorCode::last_system());
        }

        let required_size = required_size_no_null + 1;
        buffer.reserve(required_size as usize);

        if unsafe { DragQueryFileW(clipboard_data.get() as _, idx, buffer.as_mut_ptr(), required_size) == 0 } {
            return Err(ErrorCode::last_system());
        }

        unsafe {
            buffer.set_len(required_size_no_null as usize);
        }
        // 将 UTF-16 编码的切片解码为 String，将无效数据替换为 替换字符 (U+FFFD)。
        out.push(alloc::string::String::from_utf16_lossy(&buffer));
    }

    Ok(num_files as usize)
}


///Reads bitmap image, appending image to the `out` vector and returning number of bytes read on
///success.
///
///Output will contain header following by RGB
pub fn get_bitmap(out: &mut alloc::vec::Vec<u8>) -> SysResult<usize> {
    let clipboard_data = clipboard::get_clipboard_data(clipboard::CF_BITMAP)?;

    //Thanks @matheuslessarodrigues
    let mut bitmap = BITMAP {
        bmType: 0,
        bmWidth: 0,
        bmHeight: 0,
        bmWidthBytes: 0,
        bmPlanes: 0,
        bmBitsPixel: 0,
        bmBits: ptr::null_mut(),
    };

    // 检索指定图形对象的信息
    if unsafe { GetObjectW(clipboard_data.as_ptr(), mem::size_of::<BITMAP>() as _, &mut bitmap as *mut BITMAP as _) } == 0 {
        return Err(ErrorCode::last_system());
    }

    let clr_bits = bitmap.bmPlanes * bitmap.bmBitsPixel;
    let clr_bits = if clr_bits == 1 {
        1
    } else if clr_bits <= 4 {
        4
    } else if clr_bits <= 8 {
        8
    } else if clr_bits <= 16 {
        16
    } else if clr_bits <= 24 {
        24
    } else {
        32
    };

    let header_storage = RawMem::new_rust_mem(if clr_bits < 24 {
        mem::size_of::<BITMAPINFOHEADER>() + mem::size_of::<RGBQUAD>() * (1 << clr_bits)
    } else {
        mem::size_of::<BITMAPINFOHEADER>()
    })?;

    let header = unsafe {
        &mut *(header_storage.get() as *mut BITMAPINFO)
    };

    header.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as _;
    header.bmiHeader.biWidth = bitmap.bmWidth;
    header.bmiHeader.biHeight = bitmap.bmHeight;
    header.bmiHeader.biPlanes = bitmap.bmPlanes;
    header.bmiHeader.biBitCount = bitmap.bmBitsPixel;
    header.bmiHeader.biCompression = BI_RGB;
    if clr_bits < 24 {
        header.bmiHeader.biClrUsed = 1 << clr_bits;
    }

    header.bmiHeader.biSizeImage = ((((header.bmiHeader.biWidth * clr_bits + 31) & !31) / 8) * header.bmiHeader.biHeight) as _;
    header.bmiHeader.biClrImportant = 0;

    let img_size = header.bmiHeader.biSizeImage as usize;
    let out_before = out.len();

    let dc = Scope(unsafe { GetDC(ptr::null_mut()) }, free_dc);
    let mut buffer = alloc::vec::Vec::new();
    buffer.resize(img_size, 0u8);

    if unsafe { GetDIBits(dc.0, clipboard_data.as_ptr() as _, 0, bitmap.bmHeight as _, buffer.as_mut_ptr() as _, header_storage.get() as _, DIB_RGB_COLORS) } == 0 {
        return Err(ErrorCode::last_system());
    }

    //Write header
    // 克隆切片中的所有元素并将其附加到 Vec 。
    out.extend_from_slice(&u16::to_le_bytes(0x4d42)); // 以 little-endian 字节顺序将此整数的内存表示形式返回为字节数组。
    out.extend_from_slice(&u32::to_le_bytes(mem::size_of::<BITMAPFILEHEADER>() as u32 + header.bmiHeader.biSize + header.bmiHeader.biClrUsed * mem::size_of::<RGBQUAD>() as u32 + header.bmiHeader.biSizeImage));
    out.extend_from_slice(&u32::to_le_bytes(0)); //2 * u16 of 0
    out.extend_from_slice(&u32::to_le_bytes(mem::size_of::<BITMAPFILEHEADER>() as u32 + header.bmiHeader.biSize + header.bmiHeader.biClrUsed * mem::size_of::<RGBQUAD>() as u32));

    out.extend_from_slice(&header.bmiHeader.biSize.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biWidth.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biHeight.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biPlanes.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biBitCount.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biCompression.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biSizeImage.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biXPelsPerMeter.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biYPelsPerMeter.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biClrUsed.to_le_bytes());
    out.extend_from_slice(&header.bmiHeader.biClrImportant.to_le_bytes());

    for color in unsafe { slice::from_raw_parts(header.bmiColors.as_ptr(), header.bmiHeader.biClrUsed as _) } {
        out.push(color.rgbBlue);
        out.push(color.rgbGreen);
        out.push(color.rgbRed);
        out.push(color.rgbReserved);
    }

    out.extend_from_slice(&buffer);

    Ok(out.len() - out_before)
}


/// Copies raw bytes onto clipboard with specified `format`, returning whether it was successful.
///
/// This function empties the clipboard before setting the data.
pub fn set(format: u32, data: &[u8]) -> SysResult<()> {
    // 清空剪贴板内容
    let _ = empty();
    // 写内容到剪贴板
    set_without_clear(format, data)
}

#[inline]
///Empties clipboard.
///
///Wrapper around ```EmptyClipboard```.
///
///# Pre-conditions:
///
///* [open()](fn.open.html) has been called.
pub fn empty() -> SysResult<()> {
    match unsafe { EmptyClipboard() } {
        0 => Err(ErrorCode::last_system()),
        _ => Ok(()),
    }
}

/// Copies raw bytes onto the clipboard with the specified `format`, returning whether it was successful.
///
/// This function does not empty the clipboard before setting the data.
pub fn set_without_clear(format: u32, data: &[u8]) -> SysResult<()> {
    let size = data.len();
    if size == 0 {
        #[allow(clippy::unit_arg)]
        return Ok(unlikely_empty_size_result());
    }

    // 给全局内存对象分配全局内存
    let mem = RawMem::new_global_mem(size)?;

    {
        // 通过给全局内存对象加锁获得对全局内存块的引用
        let (ptr, _lock) = mem.lock()?;
        // 拷贝内容到全局内存对象
        unsafe { ptr::copy_nonoverlapping(data.as_ptr(), ptr.as_ptr() as _, size) };
        // 使用完全局内存块后需要对全局内存块解锁，这里采用了 Scope的 Drop trait
    }

    // 将全局内存对象中的内容写入到剪贴板，并释放全局内存空间
    if unsafe { !SetClipboardData(format, mem.get()).is_null() } {
        //SetClipboardData takes ownership
        mem.release();
        return Ok(());
    }

    Err(ErrorCode::last_system())
}


///Copies unicode string onto clipboard, performing necessary conversions, returning true on
///success.
pub fn set_string(data: &str) -> SysResult<()> {
    let size = unsafe {
        MultiByteToWideChar(CP_UTF8, 0, data.as_ptr() as *const _, data.len() as _, ptr::null_mut(), 0)
    };

    //MultiByteToWideChar fails on empty input, but we can ignore it and just set buffer with null char
    if size != 0 || data.is_empty() {
        let mem = RawMem::new_global_mem((mem::size_of::<u16>() * (size as usize + 1)) as _)?;
        {
            let (ptr, _lock) = mem.lock()?;
            let ptr = ptr.as_ptr() as *mut u16;
            unsafe {
                MultiByteToWideChar(CP_UTF8, 0, data.as_ptr() as *const _, data.len() as _, ptr, size);
                ptr::write(ptr.offset(size as isize), 0);
            }
        }

        let _ = empty();

        if unsafe { !SetClipboardData(clipboard::CF_UNICODETEXT, mem.get()).is_null() } {
            //SetClipboardData takes ownership
            mem.release();
            return Ok(());
        }
    }

    Err(ErrorCode::last_system())
}

#[inline(always)]
#[doc(hidden)]
pub fn set_bitamp(data: &[u8]) -> SysResult<()> {
    set_bitmap(data)
}

///Sets bitmap (header + RGB) onto clipboard, from raw bytes.
///
///Returns `ERROR_INCORRECT_SIZE` if size of data is not valid
pub fn set_bitmap(data: &[u8]) -> SysResult<()> {
    const FILE_HEADER_LEN: usize = mem::size_of::<BITMAPFILEHEADER>();
    const INFO_HEADER_LEN: usize = mem::size_of::<BITMAPINFOHEADER>();

    if data.len() <= (FILE_HEADER_LEN + INFO_HEADER_LEN) {
        return Err(ErrorCode::new_system(ERROR_INCORRECT_SIZE as _));
    }

    let mut file_header = mem::MaybeUninit::<BITMAPFILEHEADER>::uninit();
    let mut info_header = mem::MaybeUninit::<BITMAPINFOHEADER>::uninit();

    let (file_header, info_header) = unsafe {
        ptr::copy_nonoverlapping(data.as_ptr(), file_header.as_mut_ptr() as _, FILE_HEADER_LEN);
        ptr::copy_nonoverlapping(data.as_ptr().add(FILE_HEADER_LEN), info_header.as_mut_ptr() as _, INFO_HEADER_LEN);
        (file_header.assume_init(), info_header.assume_init())
    };

    if data.len() <= file_header.bfOffBits as usize {
        return Err(ErrorCode::new_system(ERROR_INCORRECT_SIZE as _));
    }

    let bitmap = &data[file_header.bfOffBits as _..];

    if bitmap.len() < info_header.biSizeImage as usize {
        return Err(ErrorCode::new_system(ERROR_INCORRECT_SIZE as _));
    }

    let dc = Scope(unsafe { GetDC(ptr::null_mut()) }, free_dc);

    let handle = unsafe {
        CreateDIBitmap(dc.0, &info_header as _, CBM_INIT, bitmap.as_ptr() as _, &info_header as *const _ as *const BITMAPINFO, DIB_RGB_COLORS)
    };

    if handle.is_null() {
        return Err(ErrorCode::last_system());
    }

    let _ = empty();
    if unsafe { SetClipboardData(clipboard::CF_BITMAP, handle as _).is_null() } {
        return Err(ErrorCode::last_system());
    }

    Ok(())
}


///Set list of file paths to clipboard.
pub fn set_file_list(paths: &[impl AsRef<str>]) -> SysResult<()> {
    #[repr(C, packed(1))]
    pub struct DROPFILES {
        pub p_files: u32,
        pub pt: POINT,
        pub f_nc: c_int,
        pub f_wide: c_int,
    }
    const DROPFILES_SIZE: DWORD = core::mem::size_of::<DROPFILES>() as DWORD;

    let mut file_list_size = 0;
    for path in paths {
        let path = path.as_ref();
        unsafe {
            //+1 for null char
            file_list_size += MultiByteToWideChar(CP_UTF8, 0, path.as_ptr() as *const _, path.len() as _, ptr::null_mut(), 0) + 1
        }
    }

    if file_list_size == 0 {
        return Err(ErrorCode::last_system());
    }

    let dropfiles = DROPFILES {
        p_files: DROPFILES_SIZE,
        pt: POINT { x: 0, y: 0 },
        f_nc: 0,
        f_wide: 1,
    };

    let mem_size = DROPFILES_SIZE as usize + (file_list_size as usize * 2) + 2; //+2 for final null char
    let mem = RawMem::new_global_mem(mem_size)?;
    {
        let (ptr, _lock) = mem.lock()?;
        let ptr = ptr.as_ptr() as *mut u8;
        unsafe {
            (ptr as *mut DROPFILES).write(dropfiles);

            let mut ptr = ptr.add(DROPFILES_SIZE as usize) as *mut u16;
            for path in paths {
                let path = path.as_ref();
                let written = MultiByteToWideChar(CP_UTF8, 0, path.as_ptr() as *const _, path.len() as _, ptr, file_list_size);
                ptr = ptr.offset(written as isize);
                //Add null termination character
                ptr.write(0);
                ptr = ptr.add(1);
                file_list_size -= written - 1;
            }
            //Add final null termination, to indicate end of list
            //null-terminate string
            ptr.write(0);
        }
    }

    let _ = empty();

    if unsafe { !SetClipboardData(clipboard::CF_HDROP, mem.get()).is_null() } {
        //SetClipboardData now has ownership of `mem`.
        mem.release();
        return Ok(());
    }
    return Err(ErrorCode::last_system());
}


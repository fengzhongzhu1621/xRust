use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};
use std::path::Path;
use std::{io, iter};

use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    MoveFileExW, ReOpenFile, SetFileAttributesW, FILE_ATTRIBUTE_NORMAL,
    FILE_ATTRIBUTE_TEMPORARY, FILE_FLAG_DELETE_ON_CLOSE, FILE_GENERIC_READ,
    FILE_GENERIC_WRITE, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
    MOVEFILE_REPLACE_EXISTING,
};

use crate::file::util;

/// 将路径添加\0结束符
fn to_utf16(s: &Path) -> Vec<u16> {
    // 两个迭代器合并
    s.as_os_str().encode_wide().chain(iter::once(0)).collect()
}

/// 创建一个新的临时文件
pub fn create_named(
    path: &Path,
    open_options: &mut OpenOptions,
) -> io::Result<File> {
    open_options
        .create_new(true)
        .read(true)
        .write(true)
        .custom_flags(FILE_ATTRIBUTE_TEMPORARY)
        .open(path)
}

/// 重新打开具有不同访问权限、共享模式和标志的指定文件系统对象。
/// https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-reopenfile
pub fn reopen(file: &File, _path: &Path) -> io::Result<File> {
    // 提取原始的文件句柄，转换为 RawHandle
    let handle = file.as_raw_handle();
    unsafe {
        let handle = ReOpenFile(
            handle as HANDLE,
            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
            FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE,
            0,
        );
        if handle == INVALID_HANDLE_VALUE {
            // 打开文件失败
            Err(io::Error::last_os_error())
        } else {
            // RawHandle -> fs::File
            Ok(FromRawHandle::from_raw_handle(handle as RawHandle))
        }
    }
}

/// 设置文件或目录的属性。
/// https://learn.microsoft.com/zh-cn/windows/win32/api/fileapi/nf-fileapi-setfileattributesw
pub fn keep(path: &Path) -> io::Result<()> {
    unsafe {
        let path_w = to_utf16(path);
        // 需要路径所在的指针，根据指针设置属性
        // FILE_ATTRIBUTE_NORMAL 取消 只读、隐藏、系统、保存
        if SetFileAttributesW(path_w.as_ptr(), FILE_ATTRIBUTE_NORMAL) == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// 使用各种移动选项移动现有文件或目录，包括其子级。
/// https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-movefileexw
pub fn persist(
    old_path: &Path,
    new_path: &Path,
    overwrite: bool,
) -> io::Result<()> {
    unsafe {
        let old_path_w = to_utf16(old_path);
        let new_path_w = to_utf16(new_path);

        // Don't succeed if this fails. We don't want to claim to have successfully persisted a file
        // still marked as temporary because this file won't have the same consistency guarantees.
        if SetFileAttributesW(old_path_w.as_ptr(), FILE_ATTRIBUTE_NORMAL) == 0
        {
            return Err(io::Error::last_os_error());
        }

        let mut flags = 0;

        if overwrite {
            // 覆盖已存在的目标文件，如果来源文件和目标文件指定的是一个目录，则不能使用此标记。
            flags |= MOVEFILE_REPLACE_EXISTING;
        }

        if MoveFileExW(old_path_w.as_ptr(), new_path_w.as_ptr(), flags) == 0 {
            let e = io::Error::last_os_error();
            // If this fails, the temporary file is now un-hidden and no longer marked temporary
            // (slightly less efficient) but it will still work.
            // 尽可能以内存作为存储数据的地方，而不是磁盘或者其他物理存储器，以减少磁盘等的I/O。
            // 用于临时存储的文件。 如果有足够的缓存内存可用，文件系统会避免将数据写回到大容量存储，
            // 因为通常情况下，应用程序在关闭句柄后会删除临时文件。
            // 在这种情况下，系统可以完全避免写入数据。 否则，在关闭句柄后写入数据。
            // 当文件句柄被关闭的时候,硬盘中的文件会立刻被删除。
            // 当程序往文件写入数据的时候,可以在资源管理器中看到该文件名,但是文件大小是 0。
            // 使用编辑器打开的时候报错:无法打开文件,相当于应用程序独占了该文件。
            // 当程序执行 CloseHandle() 之后, 资源管理器中的文件被自动删除了。
            let _ = SetFileAttributesW(
                old_path_w.as_ptr(),
                FILE_ATTRIBUTE_TEMPORARY,
            );
            Err(e)
        } else {
            Ok(())
        }
    }
}

/// 创建临时文件，文件对象的最后一个句柄被删除之后，文件将被删除
pub fn create(dir: &Path) -> io::Result<File> {
    // 创建临时文件，支持重试直到成功为止
    util::create_helper(
        dir,
        OsStr::new(".tmp"),
        OsStr::new(""),
        util::NUM_RAND_CHARS, // 随机字符的大小
        |path| {
            OpenOptions::new()
                .create_new(true)
                .read(true)
                .write(true)
                .share_mode(0)
                .custom_flags(
                    FILE_ATTRIBUTE_TEMPORARY | FILE_FLAG_DELETE_ON_CLOSE,
                )
                .open(path)
        },
    )
}

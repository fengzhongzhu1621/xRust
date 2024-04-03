// Copyright (c) 2017 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use super::error::SignalError;
use super::types::{FALSE, TRUE};
use std::io;
use std::ptr;
use windows_sys::Win32::Foundation::{
    CloseHandle, BOOL, HANDLE, WAIT_FAILED, WAIT_OBJECT_0,
};
use windows_sys::Win32::System::Console::SetConsoleCtrlHandler;
use windows_sys::Win32::System::Threading::{
    CreateSemaphoreA, ReleaseSemaphore, WaitForSingleObject, INFINITE,
};

const MAX_SEM_COUNT: i32 = 255;
static mut SEMAPHORE: HANDLE = 0 as HANDLE;

unsafe extern "system" fn os_handler(_: u32) -> BOOL {
    // Assuming this always succeeds. Can't really handle errors in any meaningful way.
    // 递增信号灯对象的计数。如果成功，就可以调用信号量上的一个等待函数（WaitForSingleObject）来减少它的计数。
    ReleaseSemaphore(SEMAPHORE, 1, ptr::null_mut());

    // 停止其它处理函数的执行
    TRUE
}

/// Register os signal handler.
///
/// Must be called before calling [`block_ctrl_c()`](fn.block_ctrl_c.html)
/// and should only be called once.
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn init_os_handler(_overwrite: bool) -> Result<(), SignalError> {
    // 创建信号量
    SEMAPHORE =
        CreateSemaphoreA(ptr::null_mut(), 0, MAX_SEM_COUNT, ptr::null());
    if SEMAPHORE == 0 {
        return Err(io::Error::last_os_error());
    }

    // 每个控制台进程都有自己的应用程序定义的 HandlerRoutine 函数的列表，这些函数可处理 Ctrl+C 和 Ctrl+Break 信号。
    // 处理程序函数还处理用户关闭控制台、注销或关闭系统时由系统生成的信号。
    // 最初，每个进程的处理程序列表仅包含调用 ExitProcess 函数的默认处理程序函数。
    // 控制台进程通过调用 SetConsoleCtrlHandler 函数（该函数不影响其他进程的处理程序函数列表）来添加或删除其他处理程序函数。
    // 当控制台进程收到任何控制信号时，将基于最后一次注册的首次调用，调用其处理程序函数，直到其中一个处理程序返回 TRUE 为止。
    // 如果所有处理程序均未返回 TRUE，则调用默认处理程序。
    if SetConsoleCtrlHandler(Some(os_handler), TRUE) == FALSE {
        let e = io::Error::last_os_error();
        CloseHandle(SEMAPHORE);
        SEMAPHORE = 0 as HANDLE;
        return Err(e);
    }

    Ok(())
}

/// Blocks until a Ctrl-C signal is received.
///
/// Must be called after calling [`init_os_handler()`](fn.init_os_handler.html).
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn block_ctrl_c() -> Result<(), SignalError> {
    // 等待 ReleaseSemaphore 发出信号
    // 每次线程完成信号灯对象的等待时，信号量对象的计数都会递减一。
    match WaitForSingleObject(SEMAPHORE, INFINITE) {
        WAIT_OBJECT_0 => Ok(()),
        WAIT_FAILED => Err(io::Error::last_os_error()),
        ret => Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "WaitForSingleObject(), unexpected return value \"{:x}\"",
                ret
            ),
        )),
    }
}

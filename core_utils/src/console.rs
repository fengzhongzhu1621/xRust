#[cfg(windows)]
#[allow(non_camel_case_types, non_snake_case)]
mod windows_console {
    use crate::platform::windows::{system::*, types::*};

    // 获得标准输出的句柄
    fn get_output_handle() -> Result<HANDLE, DWORD> {
        // This is "CONOUT$\0" UTF-16 encoded.
        const CONOUT: &[u16] =
            &[0x43, 0x4F, 0x4E, 0x4F, 0x55, 0x54, 0x24, 0x00];

        let raw_handle = unsafe {
            CreateFileW(
                CONOUT.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                core::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                core::ptr::null_mut(),
            )
        };

        if raw_handle == INVALID_HANDLE_VALUE {
            return Err(6);
        }

        Ok(raw_handle)
    }

    // 给标准输出设置颜色
    unsafe fn enable_vt(handle: HANDLE) -> Result<(), DWORD> {
        let mut dw_mode: DWORD = 0;
        if GetConsoleMode(handle, &mut dw_mode) == FALSE {
            return Err(GetLastError());
        }

        dw_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        match SetConsoleMode(handle, dw_mode) {
            result if result == TRUE => Ok(()),
            _ => Err(GetLastError()),
        }
    }

    unsafe fn enable_ansi_colors_raw() -> Result<bool, DWORD> {
        enable_vt(get_output_handle()?)?;
        Ok(true)
    }

    #[inline(always)]
    pub fn enable() -> bool {
        unsafe { enable_ansi_colors_raw().unwrap_or(false) }
    }

    // Try to enable colors on Windows, and try to do it at most once.
    pub fn cache_enable() -> bool {
        use crate::cached_bool::CachedBool;

        // 确保 enable 闭包只执行一次
        static ENABLED: CachedBool = CachedBool::new();
        ENABLED.get_or_init(enable)
    }
}

#[cfg(not(windows))]
mod windows_console {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn enable() -> bool {
        true
    }

    #[inline(always)]
    pub fn cache_enable() -> bool {
        true
    }
}

// pub use self::windows_console::enable;
pub use self::windows_console::cache_enable;

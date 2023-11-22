// unix系统，不引入 std 包
#![cfg_attr(unix, no_std)]

#[cfg(not(any(windows, target_os = "hermit", target_os = "unknown")))]
use rustix::fd::AsFd;

// 引入 windows 系统的包
#[cfg(windows)]
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle};
#[cfg(windows)]
use windows_sys::Win32::Foundation::HANDLE;

/// Extension trait to check whether something is a terminal.
pub trait IsTerminal {
    /// Returns true if this is a terminal.
    ///
    /// # Example
    ///
    /// ```
    /// use is_terminal::IsTerminal;
    ///
    /// if std::io::stdout().is_terminal() {
    ///     println!("stdout is a terminal")
    /// }
    /// ```
    fn is_terminal(&self) -> bool;
}

// T 是实现了 IsTerminal trait的对象
pub fn is_terminal<T: IsTerminal>(this: T) -> bool {
    this.is_terminal()
}

#[cfg(not(any(windows, target_os = "unknown")))]
impl<Stream: AsFd> IsTerminal for Stream {
    #[inline]
    fn is_terminal(&self) -> bool {
        #[cfg(any(unix, target_os = "wasi"))]
        {
            rustix::termios::isatty(self)
        }
    }
}

#[cfg(windows)]
impl<Stream: AsHandle> IsTerminal for Stream {
    #[inline]
    fn is_terminal(&self) -> bool {
        handle_is_console(self.as_handle())
    }
}

// The Windows implementation here is copied from `handle_is_console` in
// std/src/sys/windows/io.rs in Rust at revision
// d7b0bcb20f2f7d5f3ea3489d56ece630147e98f5.

#[cfg(windows)]
fn handle_is_console(handle: BorrowedHandle<'_>) -> bool {
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE,
        STD_OUTPUT_HANDLE,
    };

    let handle = handle.as_raw_handle();

    unsafe {
        // A null handle means the process has no console.
        if handle.is_null() {
            return false;
        }

        let mut out = 0;
        if GetConsoleMode(handle as HANDLE, &mut out) != 0 {
            // False positives aren't possible. If we got a console then we definitely have a console.
            return true;
        }

        // At this point, we *could* have a false negative. We can determine that this is a true
        // negative if we can detect the presence of a console on any of the standard I/O streams. If
        // another stream has a console, then we know we're in a Windows console and can therefore
        // trust the negative.
        for std_handle in
            [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE]
        {
            let std_handle = GetStdHandle(std_handle);
            if std_handle != 0
                && std_handle != handle as HANDLE
                && GetConsoleMode(std_handle, &mut out) != 0
            {
                return false;
            }
        }

        // Otherwise, we fall back to an msys hack to see if we can detect the presence of a pty.
        msys_tty_on(handle as HANDLE)
    }
}

/// Returns true if there is an MSYS tty on the given handle.
///
/// This incoproates d7b0bcb20f2f7d5f3ea3489d56ece630147e98f5
#[cfg(windows)]
unsafe fn msys_tty_on(handle: HANDLE) -> bool {
    use std::ffi::c_void;
    use windows_sys::Win32::{
        Foundation::MAX_PATH,
        Storage::FileSystem::{
            FileNameInfo, GetFileInformationByHandleEx, GetFileType,
            FILE_TYPE_PIPE,
        },
    };

    // Early return if the handle is not a pipe.
    if GetFileType(handle) != FILE_TYPE_PIPE {
        return false;
    }

    /// Mirrors windows_sys::Win32::Storage::FileSystem::FILE_NAME_INFO, giving
    /// it a fixed length that we can stack allocate
    #[repr(C)]
    #[allow(non_snake_case)]
    struct FILE_NAME_INFO {
        FileNameLength: u32,
        FileName: [u16; MAX_PATH as usize],
    }
    let mut name_info =
        FILE_NAME_INFO { FileNameLength: 0, FileName: [0; MAX_PATH as usize] };
    // Safety: buffer length is fixed.
    let res = GetFileInformationByHandleEx(
        handle,
        FileNameInfo,
        &mut name_info as *mut _ as *mut c_void,
        std::mem::size_of::<FILE_NAME_INFO>() as u32,
    );
    if res == 0 {
        return false;
    }

    // Use `get` because `FileNameLength` can be out of range.
    let s = match name_info
        .FileName
        .get(..name_info.FileNameLength as usize / 2)
    {
        None => return false,
        Some(s) => s,
    };
    let name = String::from_utf16_lossy(s);
    // Get the file name only.
    let name = name.rsplit('\\').next().unwrap_or(&name);
    // This checks whether 'pty' exists in the file name, which indicates that
    // a pseudo-terminal is attached. To mitigate against false positives
    // (e.g., an actual file name that contains 'pty'), we also require that
    // the file name begins with either the strings 'msys-' or 'cygwin-'.)
    let is_msys = name.starts_with("msys-") || name.starts_with("cygwin-");
    let is_pty = name.contains("-pty");
    is_msys && is_pty
}

#[cfg(target_os = "unknown")]
impl IsTerminal for std::io::Stdin {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl IsTerminal for std::io::Stdout {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl IsTerminal for std::io::Stderr {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl<'a> IsTerminal for std::io::StdinLock<'a> {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl<'a> IsTerminal for std::io::StdoutLock<'a> {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl<'a> IsTerminal for std::io::StderrLock<'a> {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl<'a> IsTerminal for std::fs::File {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl IsTerminal for std::process::ChildStdin {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl IsTerminal for std::process::ChildStdout {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(target_os = "unknown")]
impl IsTerminal for std::process::ChildStderr {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    // Verify that the msys_tty_on function works with long path.
    #[test]
    #[cfg(windows)]
    fn msys_tty_on_path_length() {
        use std::{fs::File, os::windows::io::AsRawHandle};
        use windows_sys::Win32::Foundation::MAX_PATH;

        let dir =
            tempfile::tempdir().expect("Unable to create temporary directory");
        let file_path = dir.path().join("ten_chars_".repeat(25));
        // Ensure that the path is longer than MAX_PATH.
        assert!(file_path.to_string_lossy().len() > MAX_PATH as usize);
        let file = File::create(file_path).expect("Unable to create file");

        assert!(!unsafe { crate::msys_tty_on(file.as_raw_handle() as isize) });
    }
}

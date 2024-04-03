// link_name 指定extern块中函数或静态变量的符号名称。
#[cfg(not(feature = "std"))]
extern "C" {
    /// Yeah, I had to copy this from std
    #[cfg(not(target_os = "dragonfly"))]
    #[cfg_attr(
        any(
            target_os = "linux",
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "l4re"
        ),
        link_name = "__errno_location"
    )]
    #[cfg_attr(
        any(
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "android",
            target_os = "redox",
            target_env = "newlib"
        ),
        link_name = "__errno"
    )]
    #[cfg_attr(target_os = "solaris", link_name = "___errno")]
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "freebsd"),
        link_name = "__error"
    )]
    #[cfg_attr(target_os = "haiku", link_name = "_errnop")]
    fn errno_location() -> *mut libc::c_int;
}

/// errno_location 返回与参数无关的与线程绑定的一个特定地址，应用层直接从该地址取出errno的值。
#[cfg(not(feature = "std"))]
pub fn errno() -> libc::c_int {
    unsafe { *errno_location() }
}

#[cfg(not(feature = "std"))]
#[derive(Debug)]
/// An IO error. Without std, you can only get a message or an OS error code.
pub struct Error {
    pub code: i32,
}

#[cfg(not(feature = "std"))]
impl Error {
    /// Creates an error from a raw OS error code.
    pub fn from_raw_os_error(code: i32) -> Self {
        Self { code }
    }

    /// Creates an error from the last OS error code.
    pub fn last_os_error() -> Error {
        Self::from_raw_os_error(errno() as i32)
    }

    /// Raw OS error code. Returns option for compatibility with std.
    pub fn raw_os_error(&self) -> Option<i32> {
        Some(self.code)
    }
}

#[cfg(not(feature = "std"))]
impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // 获得错误信息字符串的地址
        let msg_ptr = unsafe { libc::strerror(self.code as libc::c_int) };
        // 获得错误信息的长度
        let len = unsafe { libc::strlen(msg_ptr) };
        // 传入指向第一个元素的指针和长度参数，它会创建一个切片 &[i8]。
        // 假设传入的指针指向有效的内存，且被指向的内存具有正确的数据类型
        let slice = unsafe { slice::from_raw_parts(msg_ptr, len) };
        write!(fmt, "{}", unsafe { OsStr::from_slice(slice) })?;
        Ok(())
    }
}

/// 最新的操作系统错误码
#[cfg(feature = "std")]
pub fn errno() -> libc::c_int {
    Error::last_os_error().raw_os_error().unwrap_or(0) as libc::c_int
}

#[cfg(feature = "std")]
/// An IO error.
pub type Error = std::io::Error;

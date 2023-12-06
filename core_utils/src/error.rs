use std::path::PathBuf;
use std::{error, fmt, io};

pub fn not_supported<T>() -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "operation not supported on this platform",
    ))
}

/// 自定义路径错误
///
/// * impl std::fmt::Display Trait,并实现fmt方法
/// * 一般情况下可通过#[derive(Debug)]实现std::fmt::Debug Trait
/// * 实现std::error::Error的Trait,并根据error的级别决定是否覆盖source()方法.如果当前错误类型是低级别错误,没有子错误,则返回None,所以此时可以不覆盖soucre(); 如果当前错误有子错误,则需要覆盖该方法
#[derive(Debug)]
struct PathError {
    path: PathBuf,
    err: io::Error,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at path {:?}", self.err, self.path)
    }
}

impl error::Error for PathError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.err.source()
    }
}

pub(crate) trait IoResultExt<T> {
    fn with_err_path<F, P>(self, path: F) -> Self
    where
        F: FnOnce() -> P, // 闭包
        P: Into<PathBuf>; // 表示可以接收任意一个能够转换为 PathBuf 的类型
}

/// 将错误 Error 转换为 PathError
impl<T> IoResultExt<T> for Result<T, io::Error> {
    fn with_err_path<F, P>(self, f: F) -> Self
    where
        F: FnOnce() -> P,
        P: Into<PathBuf>,
    {
        self.map_err(|e| {
            io::Error::new(e.kind(), PathError { path: f().into(), err: e })
        })
    }
}

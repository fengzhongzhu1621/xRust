use std::error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Error {
    io_err: io::Error,
    action: String,     // 对路径的操作方式
    path: Arc<PathBuf>, // 线程安全的引用计数智能指针
}

impl Error {
    /// Create a new error when the path and action are known.
    pub fn new(io_err: io::Error, action: &str, path: Arc<PathBuf>) -> Error {
        Error { io_err, action: action.into(), path }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} when {} {}",
            self.io_err,
            self.action,
            self.path.display()
        )
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error<{}>", self)
    }
}

impl Error {
    /// Returns the path associated with this error.
    /// Arc<PathBuf> -> &PathBuf
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// Returns the `std::io::Error` associated with this errors.
    pub fn io_error(&self) -> &io::Error {
        &self.io_err
    }

    /// Returns the action being performed when this error occured.
    pub fn action(&self) -> &str {
        &self.action
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.io_err.description()
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        Some(&self.io_err)
    }
}

/// Error -> io::Error
impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(err.io_err.kind(), err)
    }
}

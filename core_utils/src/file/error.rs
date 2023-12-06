//! 自定义路径错误

use super::temp_file::NamedTempFile;
use super::temp_path::TempPath;
use std::error;
use std::fs::File;
use std::{fmt, io};

/// 自定义临时路径错误
/// Error returned when persisting a temporary file path fails.
#[derive(Debug)]
pub struct PathPersistError {
    /// The underlying IO error.
    pub error: io::Error,
    /// The temporary file path that couldn't be persisted.
    pub path: TempPath,
}

impl fmt::Display for PathPersistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to persist temporary file path: {}", self.error)
    }
}

impl error::Error for PathPersistError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}

/// PathPersistError -> io::Error
impl From<PathPersistError> for io::Error {
    #[inline]
    fn from(error: PathPersistError) -> io::Error {
        error.error
    }
}

/// PathPersistError -> TempPath
impl From<PathPersistError> for TempPath {
    #[inline]
    fn from(error: PathPersistError) -> TempPath {
        error.path
    }
}

/// Error returned when persisting a temporary file fails.
/// 指定 F 的类型
pub struct PersistError<F = File> {
    /// The underlying IO error.
    pub error: io::Error,
    /// The temporary file that couldn't be persisted.
    pub file: NamedTempFile<F>,
}

impl<F> fmt::Display for PersistError<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to persist temporary file: {}", self.error)
    }
}

impl<F> error::Error for PersistError<F> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}

impl<F> fmt::Debug for PersistError<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PersistError({:?})", self.error)
    }
}

// PersistError -> io::Error
impl<F> From<PersistError<F>> for io::Error {
    #[inline]
    fn from(error: PersistError<F>) -> io::Error {
        error.error
    }
}

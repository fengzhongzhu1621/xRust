use super::builder::Builder;
use super::error::{PathPersistError, PersistError};
use super::imp;
use super::temp_path::TempPath;
use crate::error::IoResultExt;
use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsFd, AsRawFd, BorrowedFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, RawHandle};

/// 在默认目录创建临时文件
pub fn tempfile() -> io::Result<File> {
    tempfile_in(env::temp_dir())
}

/// 创建临时文件
/// Create a new temporary file in the specified directory.
/// AsRef是一个用于实现引用转换的特型(trait)，AsRef<T>相当于&T。
/// P 类型实现了 AsRef<Path> 特性
pub fn tempfile_in<P: AsRef<Path>>(dir: P) -> io::Result<File> {
    imp::create(dir.as_ref())
}

pub(crate) fn create_named(
    mut path: PathBuf,
    open_options: &mut OpenOptions,
) -> io::Result<NamedTempFile> {
    // Make the path absolute. Otherwise, changing directories could cause us to
    // delete the wrong file.
    if !path.is_absolute() {
        path = env::current_dir()?.join(path)
    }
    // 根据路径创建文件，如果错误则将错误转换为 PathError
    imp::create_named(&path, open_options).with_err_path(|| path.clone()).map(
        |file| NamedTempFile {
            path: TempPath { path: path.into_boxed_path() },
            file,
        },
    )
}

/// A named temporary file.
pub struct NamedTempFile<F = File> {
    path: TempPath,
    file: F,
}

impl<F> fmt::Debug for NamedTempFile<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NamedTempFile({:?})", self.path)
    }
}

// NamedTempFile -> &Path
impl<F> AsRef<Path> for NamedTempFile<F> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

// PersistError<F = File> -> NamedTempFile<F>
impl<F> From<PersistError<F>> for NamedTempFile<F> {
    #[inline]
    fn from(error: PersistError<F>) -> NamedTempFile<F> {
        error.file
    }
}

impl NamedTempFile<File> {
    /// Create a new named temporary file.
    pub fn new() -> io::Result<NamedTempFile> {
        Builder::new().tempfile()
    }

    /// Create a new named temporary file in the specified directory.
    pub fn new_in<P: AsRef<Path>>(dir: P) -> io::Result<NamedTempFile> {
        Builder::new().tempfile_in(dir)
    }

    /// Create a new named temporary file with the specified filename prefix.
    pub fn with_prefix<S: AsRef<OsStr>>(
        prefix: S,
    ) -> io::Result<NamedTempFile> {
        Builder::new().prefix(&prefix).tempfile()
    }
    /// Create a new named temporary file with the specified filename prefix,
    /// in the specified directory.
    pub fn with_prefix_in<S: AsRef<OsStr>, P: AsRef<Path>>(
        prefix: S,
        dir: P,
    ) -> io::Result<NamedTempFile> {
        Builder::new().prefix(&prefix).tempfile_in(dir)
    }
}

impl<F> NamedTempFile<F> {
    /// Get the temporary file's path.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Close and remove the temporary file.
    pub fn close(self) -> io::Result<()> {
        let NamedTempFile { path, .. } = self;
        path.close()
    }

    /// Persist the temporary file at the target path.
    pub fn persist<P: AsRef<Path>>(
        self,
        new_path: P,
    ) -> Result<F, PersistError<F>> {
        let NamedTempFile { path, file } = self;
        match path.persist(new_path) {
            Ok(_) => Ok(file),
            Err(err) => {
                let PathPersistError { error, path } = err;
                Err(PersistError { file: NamedTempFile { path, file }, error })
            }
        }
    }

    /// Persist the temporary file at the target path if and only if no file exists there.
    pub fn persist_noclobber<P: AsRef<Path>>(
        self,
        new_path: P,
    ) -> Result<F, PersistError<F>> {
        let NamedTempFile { path, file } = self;
        match path.persist_noclobber(new_path) {
            Ok(_) => Ok(file),
            Err(err) => {
                let PathPersistError { error, path } = err;
                Err(PersistError { file: NamedTempFile { path, file }, error })
            }
        }
    }

    /// Keep the temporary file from being deleted. This function will turn the
    /// temporary file into a non-temporary file without moving it.
    pub fn keep(self) -> Result<(F, PathBuf), PersistError<F>> {
        let (file, path) = (self.file, self.path);
        match path.keep() {
            Ok(path) => Ok((file, path)),
            Err(PathPersistError { error, path }) => {
                Err(PersistError { file: NamedTempFile { path, file }, error })
            }
        }
    }

    /// Get a reference to the underlying file.
    pub fn as_file(&self) -> &F {
        &self.file
    }

    /// Get a mutable reference to the underlying file.
    pub fn as_file_mut(&mut self) -> &mut F {
        &mut self.file
    }

    /// Convert the temporary file into a `std::fs::File`.
    ///
    /// The inner file will be deleted.
    pub fn into_file(self) -> F {
        self.file
    }

    /// Closes the file, leaving only the temporary file path.
    pub fn into_temp_path(self) -> TempPath {
        self.path
    }

    /// Converts the named temporary file into its constituent parts.
    pub fn into_parts(self) -> (F, TempPath) {
        (self.file, self.path)
    }

    /// Creates a `NamedTempFile` from its constituent parts.
    pub fn from_parts(file: F, path: TempPath) -> Self {
        Self { file, path }
    }
}

impl NamedTempFile<File> {
    /// Securely reopen the temporary file.
    pub fn reopen(&self) -> io::Result<File> {
        imp::reopen(self.as_file(), NamedTempFile::path(self))
            .with_err_path(|| NamedTempFile::path(self))
    }
}

impl<F: Read> Read for NamedTempFile<F> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_file_mut().read(buf).with_err_path(|| self.path())
    }

    fn read_vectored(
        &mut self,
        bufs: &mut [io::IoSliceMut<'_>],
    ) -> io::Result<usize> {
        self.as_file_mut().read_vectored(bufs).with_err_path(|| self.path())
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.as_file_mut().read_to_end(buf).with_err_path(|| self.path())
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.as_file_mut().read_to_string(buf).with_err_path(|| self.path())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.as_file_mut().read_exact(buf).with_err_path(|| self.path())
    }
}

impl Read for &NamedTempFile<File> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_file().read(buf).with_err_path(|| self.path())
    }

    fn read_vectored(
        &mut self,
        bufs: &mut [io::IoSliceMut<'_>],
    ) -> io::Result<usize> {
        self.as_file().read_vectored(bufs).with_err_path(|| self.path())
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.as_file().read_to_end(buf).with_err_path(|| self.path())
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.as_file().read_to_string(buf).with_err_path(|| self.path())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.as_file().read_exact(buf).with_err_path(|| self.path())
    }
}

impl<F: Write> Write for NamedTempFile<F> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_file_mut().write(buf).with_err_path(|| self.path())
    }
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.as_file_mut().flush().with_err_path(|| self.path())
    }

    fn write_vectored(
        &mut self,
        bufs: &[io::IoSlice<'_>],
    ) -> io::Result<usize> {
        self.as_file_mut().write_vectored(bufs).with_err_path(|| self.path())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.as_file_mut().write_all(buf).with_err_path(|| self.path())
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.as_file_mut().write_fmt(fmt).with_err_path(|| self.path())
    }
}

impl Write for &NamedTempFile<File> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_file().write(buf).with_err_path(|| self.path())
    }
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.as_file().flush().with_err_path(|| self.path())
    }

    fn write_vectored(
        &mut self,
        bufs: &[io::IoSlice<'_>],
    ) -> io::Result<usize> {
        self.as_file().write_vectored(bufs).with_err_path(|| self.path())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.as_file().write_all(buf).with_err_path(|| self.path())
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.as_file().write_fmt(fmt).with_err_path(|| self.path())
    }
}

impl<F: Seek> Seek for NamedTempFile<F> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.as_file_mut().seek(pos).with_err_path(|| self.path())
    }
}

impl Seek for &NamedTempFile<File> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.as_file().seek(pos).with_err_path(|| self.path())
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<F: AsFd> AsFd for NamedTempFile<F> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.as_file().as_fd()
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<F: AsRawFd> AsRawFd for NamedTempFile<F> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.as_file().as_raw_fd()
    }
}

#[cfg(windows)]
impl<F: AsHandle> AsHandle for NamedTempFile<F> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.as_file().as_handle()
    }
}

#[cfg(windows)]
impl<F: AsRawHandle> AsRawHandle for NamedTempFile<F> {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.as_file().as_raw_handle()
    }
}

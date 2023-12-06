use std::ffi::OsStr;
use std::fs::remove_dir_all;
use std::mem;
use std::path::{self, Path, PathBuf};
use std::{fmt, fs, io};

use super::builder::Builder;
use crate::error::IoResultExt;

/// Create a new temporary directory.
pub fn tempdir() -> io::Result<TempDir> {
    TempDir::new()
}

/// Create a new temporary directory in a specific directory.
pub struct TempDir {
    path: Box<Path>,
}

impl TempDir {
    /// Attempts to make a temporary directory inside of `env::temp_dir()`.
    pub fn new() -> io::Result<TempDir> {
        Builder::new().tempdir()
    }

    /// Attempts to make a temporary directory inside of `dir`.
    /// The directory and everything inside it will be automatically
    /// deleted once the returned `TempDir` is destroyed.
    pub fn new_in<P: AsRef<Path>>(dir: P) -> io::Result<TempDir> {
        Builder::new().tempdir_in(dir)
    }

    /// Attempts to make a temporary directory with the specified prefix inside of
    /// `env::temp_dir()`. The directory and everything inside it will be automatically
    /// deleted once the returned `TempDir` is destroyed.
    pub fn with_prefix<S: AsRef<OsStr>>(prefix: S) -> io::Result<TempDir> {
        Builder::new().prefix(&prefix).tempdir()
    }

    /// Attempts to make a temporary directory with the specified prefix inside
    /// the specified directory. The directory and everything inside it will be
    /// automatically deleted once the returned `TempDir` is destroyed.
    pub fn with_prefix_in<S: AsRef<OsStr>, P: AsRef<Path>>(
        prefix: S,
        dir: P,
    ) -> io::Result<TempDir> {
        Builder::new().prefix(&prefix).tempdir_in(dir)
    }

    /// Accesses the [`Path`] to the temporary directory.
    #[must_use]
    pub fn path(&self) -> &path::Path {
        self.path.as_ref()
    }

    /// Persist the temporary directory to disk, returning the [`PathBuf`] where it is located.
    #[must_use]
    pub fn into_path(self) -> PathBuf {
        // Prevent the Drop impl from being called.
        let mut this = mem::ManuallyDrop::new(self);

        // replace this.path with an empty Box, since an empty Box does not
        // allocate any heap memory.
        mem::replace(&mut this.path, PathBuf::new().into_boxed_path()).into()
    }

    /// Closes and removes the temporary directory, returning a `Result`.
    pub fn close(mut self) -> io::Result<()> {
        let result = remove_dir_all(self.path()).with_err_path(|| self.path());

        // Set self.path to empty Box to release the memory, since an empty
        // Box does not allocate any heap memory.
        self.path = PathBuf::new().into_boxed_path();

        // Prevent the Drop impl from being called.
        mem::forget(self);

        result
    }
}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

impl fmt::Debug for TempDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TempDir").field("path", &self.path()).finish()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = remove_dir_all(self.path());
    }
}

pub(crate) fn create(path: PathBuf) -> io::Result<TempDir> {
    fs::create_dir(&path)
        .with_err_path(|| &path)
        .map(|_| TempDir { path: path.into_boxed_path() })
}

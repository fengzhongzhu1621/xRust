use super::{PathInfo, PathMut, Result};
use std::ffi;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub trait PathOps: PathInfo {
    type Output: PathOps;

    /// Returns a new value representing the concatenation of two paths.
    fn concat<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output>;

    /// An exact replica of `std::path::Path::join` with all of its gotchas and pitfalls,, except
    /// returns a more relevant type.
    fn join<P: AsRef<Path>>(&self, path: P) -> Self::Output;

    /// Creates a new path object like `self` but with the given file name.
    fn with_file_name<S: AsRef<ffi::OsStr>>(
        &self,
        file_name: S,
    ) -> Self::Output;

    /// Creates a new path object like `self` but with the given extension./// [`std::path::Path::with_extension()`]: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.with_extension
    fn with_extension<S: AsRef<ffi::OsStr>>(
        &self,
        extension: S,
    ) -> Self::Output;
}

impl PathOps for Path {
    type Output = PathBuf;

    fn concat<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output> {
        // &Path -> PathBuf
        let mut res = self.to_owned();
        res.append(path)?;
        Ok(res)
    }

    fn join<P: AsRef<Path>>(&self, path: P) -> Self::Output {
        Path::join(self, path)
    }

    fn with_file_name<S: AsRef<ffi::OsStr>>(
        &self,
        file_name: S,
    ) -> Self::Output {
        let mut res = self.to_owned();
        res.set_file_name(file_name);
        res
    }

    fn with_extension<S: AsRef<ffi::OsStr>>(
        &self,
        extension: S,
    ) -> Self::Output {
        let mut res = self.to_owned();
        res.set_extension(extension);
        res
    }
}

impl PathOps for PathBuf {
    type Output = PathBuf;

    fn concat<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output> {
        self.as_path().concat(path)
    }

    fn join<P: AsRef<Path>>(&self, path: P) -> Self::Output {
        Path::join(self, path)
    }

    fn with_file_name<S: AsRef<ffi::OsStr>>(
        &self,
        file_name: S,
    ) -> Self::Output {
        self.as_path().with_file_name(file_name)
    }

    fn with_extension<S: AsRef<ffi::OsStr>>(
        &self,
        extension: S,
    ) -> Self::Output {
        self.as_path().with_extension(extension)
    }
}

impl PathOps for Arc<PathBuf> {
    type Output = Arc<PathBuf>;

    fn concat<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output> {
        let mut res = self.clone();
        Arc::make_mut(&mut res).append(path)?;
        Ok(res)
    }

    fn join<P: AsRef<Path>>(&self, path: P) -> Self::Output {
        let buf = Path::join(self, path);
        Arc::new(buf)
    }

    fn with_file_name<S: AsRef<ffi::OsStr>>(
        &self,
        file_name: S,
    ) -> Self::Output {
        let mut res = self.clone();
        Arc::make_mut(&mut res).set_file_name(file_name);
        res
    }

    fn with_extension<S: AsRef<ffi::OsStr>>(
        &self,
        extension: S,
    ) -> Self::Output {
        let mut res = self.clone();
        Arc::make_mut(&mut res).set_extension(extension);
        res
    }
}

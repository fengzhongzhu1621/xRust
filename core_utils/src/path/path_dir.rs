use super::{Error, Result};
use super::{PathAbs, PathInfo, PathOps, PathType};
use std::borrow::Borrow;
use std::ffi;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// A `PathAbs` that is guaranteed to be a directory, with associated methods.
pub struct PathDir(pub(crate) PathAbs);

impl PathDir {
    /// Instantiate a new `PathDir`. The directory must exist or `io::Error` will be returned.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<PathDir> {
        let abs = PathAbs::new(path)?;
        PathDir::try_from(abs)
    }

    pub fn new_unchecked<P: Into<Arc<PathBuf>>>(path: P) -> PathDir {
        PathDir(PathAbs::new_unchecked(path))
    }

    pub fn try_from<P: Into<PathAbs>>(path: P) -> Result<PathDir> {
        let abs = path.into();
        if abs.is_dir() {
            Ok(PathDir::new_unchecked(abs))
        } else {
            Err(Error::new(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "path is not a dir",
                ),
                "resolving",
                abs.into(),
            ))
        }
    }

    pub fn current_dir() -> Result<PathDir> {
        let dir = ::std::env::current_dir().map_err(|err| {
            Error::new(
                err,
                "getting current_dir",
                Path::new("$CWD").to_path_buf().into(),
            )
        })?;
        PathDir::new(dir)
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<PathDir> {
        if let Err(err) = fs::create_dir(&path) {
            match err.kind() {
                io::ErrorKind::AlreadyExists => {}
                _ => {
                    return Err(Error::new(
                        err,
                        "creating",
                        path.as_ref().to_path_buf().into(),
                    ));
                }
            }
        }
        PathDir::new(path)
    }

    pub fn create_all<P: AsRef<Path>>(path: P) -> Result<PathDir> {
        fs::create_dir_all(&path).map_err(|err| {
            Error::new(err, "creating-all", path.as_ref().to_path_buf().into())
        })?;
        PathDir::new(path)
    }

    pub fn list(&self) -> Result<ListDir> {
        let fsread = fs::read_dir(self).map_err(|err| {
            Error::new(err, "reading dir", self.clone().into())
        })?;
        Ok(ListDir { dir: self.clone(), fsread: fsread })
    }

    pub fn remove(self) -> Result<()> {
        fs::remove_dir(&self)
            .map_err(|err| Error::new(err, "removing", self.into()))
    }

    pub fn remove_all(self) -> Result<()> {
        fs::remove_dir_all(&self)
            .map_err(|err| Error::new(err, "removing-all", self.into()))
    }

    pub fn symlink<P: AsRef<Path>>(&self, dst: P) -> Result<PathDir> {
        symlink_dir(&self, &dst).map_err(|err| {
            Error::new(
                err,
                &format!("linking from {} to", dst.as_ref().display()),
                self.clone().into(),
            )
        })?;
        PathDir::new(dst)
    }

    pub fn as_path(&self) -> &Path {
        self.as_ref()
    }

    pub fn canonicalize(&self) -> Result<PathDir> {
        Ok(PathDir(self.0.canonicalize()?))
    }

    pub fn parent_dir(&self) -> Option<PathDir> {
        match self.parent() {
            Ok(path) => Some(PathDir(PathAbs(Arc::new(path.to_path_buf())))),
            Err(_) => None,
        }
    }
}

impl PathOps for PathDir {
    type Output = PathAbs;

    fn concat<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output> {
        Ok(self.0.concat(path)?)
    }

    fn join<P: AsRef<Path>>(&self, path: P) -> Self::Output {
        let buf = Path::join(self.as_path(), path);
        Self::Output::new_unchecked(buf)
    }

    fn with_file_name<S: AsRef<ffi::OsStr>>(
        &self,
        file_name: S,
    ) -> Self::Output {
        self.0.with_file_name(file_name)
    }

    fn with_extension<S: AsRef<ffi::OsStr>>(
        &self,
        extension: S,
    ) -> Self::Output {
        self.0.with_extension(extension)
    }
}

impl fmt::Debug for PathDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<ffi::OsStr> for PathDir {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl AsRef<PathAbs> for PathDir {
    fn as_ref(&self) -> &PathAbs {
        &self.0
    }
}

impl AsRef<Path> for PathDir {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<PathBuf> for PathDir {
    fn as_ref(&self) -> &PathBuf {
        self.0.as_ref()
    }
}

impl Borrow<PathAbs> for PathDir {
    fn borrow(&self) -> &PathAbs {
        self.as_ref()
    }
}

impl Borrow<Path> for PathDir {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl Borrow<PathBuf> for PathDir {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl<'a> Borrow<PathAbs> for &'a PathDir {
    fn borrow(&self) -> &PathAbs {
        self.as_ref()
    }
}

impl<'a> Borrow<Path> for &'a PathDir {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl<'a> Borrow<PathBuf> for &'a PathDir {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl From<PathDir> for PathAbs {
    fn from(path: PathDir) -> PathAbs {
        path.0
    }
}

impl From<PathDir> for Arc<PathBuf> {
    fn from(path: PathDir) -> Arc<PathBuf> {
        let abs: PathAbs = path.into();
        abs.into()
    }
}

impl From<PathDir> for PathBuf {
    fn from(path: PathDir) -> PathBuf {
        let abs: PathAbs = path.into();
        abs.into()
    }
}

#[cfg(target_os = "wasi")]
fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> io::Result<()> {
    std::os::wasi::fs::symlink_path(src, dst)
}

#[cfg(unix)]
fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> io::Result<()> {
    ::std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> io::Result<()> {
    ::std::os::windows::fs::symlink_dir(src, dst)
}

impl PathDir {
    pub fn join_abs<P: AsRef<Path>>(&self, path: P) -> Result<PathType> {
        let joined = self.concat(path.as_ref())?;
        PathType::new(joined)
    }
}

/// An iterator over `PathType` objects, returned by `PathDir::list`.
pub struct ListDir {
    // TODO: this should be a reference...?
    // Or is this a good excuse to use Arc under the hood everywhere?
    dir: PathDir,
    fsread: fs::ReadDir,
}

impl ::std::iter::Iterator for ListDir {
    type Item = Result<PathType>;
    fn next(&mut self) -> Option<Result<PathType>> {
        let entry = match self.fsread.next() {
            Some(r) => match r {
                Ok(e) => e,
                Err(err) => {
                    return Some(Err(Error::new(
                        err,
                        "iterating over",
                        self.dir.clone().into(),
                    )));
                }
            },
            None => return None,
        };
        Some(PathType::new(entry.path()))
    }
}

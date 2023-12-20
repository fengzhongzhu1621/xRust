use super::{Error, Result};
use super::{
    FileEdit, FileRead, FileWrite, PathAbs, PathDir, PathInfo, PathOps,
};
use std::borrow::Borrow;
use std::ffi;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// a `PathAbs` that was a file at the time of initialization, with associated methods.
pub struct PathFile(pub(crate) PathAbs);

impl PathFile {
    /// Instantiate a new `PathFile`. The file must exist or `io::Error` will be returned.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<PathFile> {
        let abs = PathAbs::new(path)?;
        PathFile::try_from(abs)
    }

    pub fn new_unchecked<P: Into<Arc<PathBuf>>>(path: P) -> PathFile {
        PathFile(PathAbs::new_unchecked(path))
    }

    /// Convert a `PathAbs` into a `PathFile`, first validating that the path is a file.
    pub fn try_from<P: Into<PathAbs>>(path: P) -> Result<PathFile> {
        let abs = path.into();
        if abs.is_file() {
            Ok(PathFile::new_unchecked(abs))
        } else {
            Err(Error::new(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "path is not a file",
                ),
                "resolving",
                abs.into(),
            ))
        }
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<PathFile> {
        fs::OpenOptions::new().write(true).create(true).open(&path).map_err(
            |err| {
                Error::new(err, "opening", path.as_ref().to_path_buf().into())
            },
        )?;
        PathFile::new(path)
    }

    pub fn copy<P: AsRef<Path>>(&self, path: P) -> Result<PathFile> {
        fs::copy(&self, &path).map_err(|err| {
            Error::new(
                err,
                &format!("copying {} from", path.as_ref().display()),
                self.clone().into(),
            )
        })?;
        Ok(PathFile::new(path)?)
    }

    pub fn rename<P: AsRef<Path>>(self, to: P) -> Result<PathFile> {
        fs::rename(&self, &to).map_err(|err| {
            Error::new(
                err,
                &format!("renaming to {} from", to.as_ref().display()),
                self.clone().into(),
            )
        })?;
        Ok(PathFile::new(to)?)
    }

    pub fn symlink<P: AsRef<Path>>(&self, dst: P) -> Result<PathFile> {
        symlink_file(&self, &dst).map_err(|err| {
            Error::new(
                err,
                &format!("linking from {} to", dst.as_ref().display()),
                self.clone().into(),
            )
        })?;
        PathFile::new(dst)
    }

    pub fn remove(self) -> Result<()> {
        fs::remove_file(&self)
            .map_err(|err| Error::new(err, "removing", self.into()))
    }

    /// Return a reference to a basic `std::path::Path`
    pub fn as_path(&self) -> &Path {
        self.as_ref()
    }

    pub fn canonicalize(&self) -> Result<PathFile> {
        Ok(PathFile(self.0.canonicalize()?))
    }
}

impl fmt::Debug for PathFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<ffi::OsStr> for PathFile {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl AsRef<PathAbs> for PathFile {
    fn as_ref(&self) -> &PathAbs {
        &self.0
    }
}

impl AsRef<Path> for PathFile {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<PathBuf> for PathFile {
    fn as_ref(&self) -> &PathBuf {
        self.0.as_ref()
    }
}

impl Borrow<PathAbs> for PathFile {
    fn borrow(&self) -> &PathAbs {
        self.as_ref()
    }
}

impl Borrow<Path> for PathFile {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl Borrow<PathBuf> for PathFile {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl<'a> Borrow<PathAbs> for &'a PathFile {
    fn borrow(&self) -> &PathAbs {
        self.as_ref()
    }
}

impl<'a> Borrow<Path> for &'a PathFile {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl<'a> Borrow<PathBuf> for &'a PathFile {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl From<PathFile> for PathAbs {
    fn from(path: PathFile) -> PathAbs {
        path.0
    }
}

impl From<PathFile> for Arc<PathBuf> {
    fn from(path: PathFile) -> Arc<PathBuf> {
        let abs: PathAbs = path.into();
        abs.into()
    }
}

impl From<PathFile> for PathBuf {
    fn from(path: PathFile) -> PathBuf {
        let abs: PathAbs = path.into();
        abs.into()
    }
}

impl PathOps for PathFile {
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

#[cfg(target_os = "wasi")]
fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> io::Result<()> {
    std::os::wasi::fs::symlink_path(src, dst)
}

#[cfg(unix)]
fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> io::Result<()> {
    ::std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> io::Result<()> {
    ::std::os::windows::fs::symlink_file(src, dst)
}

impl PathFile {
    pub fn read_string(&self) -> Result<String> {
        let mut f = self.open_read()?;
        f.read_string()
    }

    pub fn open_read(&self) -> Result<FileRead> {
        FileRead::open_abs(self.clone())
    }
}

impl PathFile {
    pub fn write_str(&self, s: &str) -> Result<()> {
        let mut options = fs::OpenOptions::new();
        options.create(true);
        options.truncate(true);
        let mut f = FileWrite::open_abs(self.clone(), options)?;
        if s.is_empty() {
            return Ok(());
        }
        f.write_str(s)?;
        f.flush()
    }

    pub fn append_str(&self, s: &str) -> Result<()> {
        let mut f = self.open_append()?;
        if s.is_empty() {
            return Ok(());
        }
        f.write_str(s)?;
        f.flush()
    }

    pub fn open_append(&self) -> Result<FileWrite> {
        let mut options = fs::OpenOptions::new();
        options.append(true);
        FileWrite::open_abs(self.clone(), options)
    }
}

impl PathFile {
    pub fn open_edit(&self) -> Result<FileEdit> {
        FileEdit::open_abs(self.clone(), fs::OpenOptions::new())
    }
}

impl PathFile {
    pub fn parent_dir(&self) -> PathDir {
        let path = self.parent().expect("PathFile did not have a parent.");
        PathDir::new_unchecked(PathAbs::new_unchecked(path.to_path_buf()))
    }
}

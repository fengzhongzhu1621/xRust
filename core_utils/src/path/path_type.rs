use super::Result;
use super::{PathAbs, PathDir, PathFile, PathInfo, PathOps};
use std::borrow::Borrow;
use std::ffi;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(tag = "type", content = "path", rename_all = "lowercase")
)]
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// An an enum containing either a file or a directory.
///
/// This is used primarily for:
/// - The items returned from `PathDir::list`
/// - Serializing paths of different types.
///
/// Note that for symlinks, this returns the underlying file type.
pub enum PathType {
    File(PathFile),
    Dir(PathDir),
}

impl PathType {
    /// Resolves and returns the `PathType` of the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<PathType> {
        let abs = PathAbs::new(&path)?;
        PathType::try_from(abs)
    }

    /// Consume the `PathAbs` returning the `PathType`.
    pub fn try_from<P: Into<PathAbs>>(path: P) -> Result<PathType> {
        let abs = path.into();
        let ty = abs.metadata()?.file_type();
        if ty.is_file() {
            Ok(PathType::File(PathFile(abs)))
        } else if ty.is_dir() {
            Ok(PathType::Dir(PathDir(abs)))
        } else {
            unreachable!(
                "rust docs: The fs::metadata function follows symbolic links"
            )
        }
    }

    pub fn unwrap_file(self) -> PathFile {
        match self {
            PathType::File(f) => f,
            PathType::Dir(d) => {
                panic!(
                    "unwrap_file called on {}, which is not a file",
                    d.display()
                )
            }
        }
    }

    pub fn unwrap_dir(self) -> PathDir {
        match self {
            PathType::Dir(d) => d,
            PathType::File(f) => panic!(
                "unwrap_dir called on {}, which is not a directory",
                f.display()
            ),
        }
    }

    /// Return whether this variant is `PathType::Dir`.
    pub fn is_dir(&self) -> bool {
        if let PathType::Dir(_) = *self {
            true
        } else {
            false
        }
    }

    /// Return whether this variant is `PathType::File`.
    pub fn is_file(&self) -> bool {
        if let PathType::File(_) = *self {
            true
        } else {
            false
        }
    }
}

impl AsRef<ffi::OsStr> for PathType {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_path().as_ref()
    }
}

impl AsRef<PathAbs> for PathType {
    fn as_ref(&self) -> &PathAbs {
        match *self {
            PathType::File(ref file) => file.as_ref(),
            PathType::Dir(ref dir) => dir.as_ref(),
        }
    }
}

impl AsRef<Path> for PathType {
    fn as_ref(&self) -> &Path {
        let r: &PathAbs = self.as_ref();
        r.as_ref()
    }
}

impl AsRef<PathBuf> for PathType {
    fn as_ref(&self) -> &PathBuf {
        let r: &PathAbs = self.as_ref();
        r.as_ref()
    }
}

impl Borrow<PathAbs> for PathType {
    fn borrow(&self) -> &PathAbs {
        self.as_ref()
    }
}

impl Borrow<Path> for PathType {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl Borrow<PathBuf> for PathType {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl<'a> Borrow<PathAbs> for &'a PathType {
    fn borrow(&self) -> &PathAbs {
        self.as_ref()
    }
}

impl<'a> Borrow<Path> for &'a PathType {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl<'a> Borrow<PathBuf> for &'a PathType {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl From<PathType> for PathAbs {
    fn from(path: PathType) -> PathAbs {
        match path {
            PathType::File(p) => p.into(),
            PathType::Dir(p) => p.into(),
        }
    }
}

impl From<PathType> for Arc<PathBuf> {
    fn from(path: PathType) -> Arc<PathBuf> {
        let abs: PathAbs = path.into();
        abs.into()
    }
}

impl From<PathType> for PathBuf {
    fn from(path: PathType) -> PathBuf {
        let abs: PathAbs = path.into();
        abs.into()
    }
}

impl PathOps for PathType {
    type Output = PathAbs;

    fn concat<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output> {
        match self {
            PathType::File(p) => p.concat(path),
            PathType::Dir(p) => p.concat(path),
        }
    }

    fn join<P: AsRef<Path>>(&self, path: P) -> Self::Output {
        let buf = Path::join(self.as_path(), path);
        Self::Output::new_unchecked(buf)
    }

    fn with_file_name<S: AsRef<ffi::OsStr>>(
        &self,
        file_name: S,
    ) -> Self::Output {
        match self {
            PathType::File(p) => p.with_file_name(file_name),
            PathType::Dir(p) => p.with_file_name(file_name),
        }
    }

    fn with_extension<S: AsRef<ffi::OsStr>>(
        &self,
        extension: S,
    ) -> Self::Output {
        match self {
            PathType::File(p) => p.with_extension(extension),
            PathType::Dir(p) => p.with_extension(extension),
        }
    }
}

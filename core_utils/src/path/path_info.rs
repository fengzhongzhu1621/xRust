use super::{Error, PathAbs, Result};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::ffi;
use std::fs;
use std::io;
use std::path::{self, Components, Path, PathBuf};
use std::sync::Arc;

pub trait PathInfo {
    fn as_path(&self) -> &Path;

    fn to_arc_pathbuf(&self) -> Arc<PathBuf>;

    fn as_os_str(&self) -> &ffi::OsStr {
        Path::as_os_str(self.as_path())
    }

    fn to_str(&self) -> Option<&str> {
        Path::to_str(self.as_path())
    }

    fn to_string_lossy(&self) -> Cow<'_, str> {
        Path::to_string_lossy(self.as_path())
    }

    fn is_absolute(&self) -> bool {
        Path::is_absolute(self.as_path())
    }

    fn is_relative(&self) -> bool {
        Path::is_relative(self.as_path())
    }

    fn has_root(&self) -> bool {
        Path::has_root(self.as_path())
    }

    fn ancestors(&self) -> path::Ancestors<'_> {
        Path::ancestors(self.as_path())
    }

    fn file_name(&self) -> Option<&ffi::OsStr> {
        Path::file_name(self.as_path())
    }

    fn strip_prefix<P>(
        &self,
        base: P,
    ) -> std::result::Result<&Path, path::StripPrefixError>
    where
        P: AsRef<Path>,
    {
        Path::strip_prefix(self.as_path(), base)
    }

    fn starts_with<P: AsRef<Path>>(&self, base: P) -> bool {
        Path::starts_with(self.as_path(), base)
    }

    fn ends_with<P: AsRef<Path>>(&self, base: P) -> bool {
        Path::ends_with(self.as_path(), base)
    }

    fn file_stem(&self) -> Option<&ffi::OsStr> {
        Path::file_stem(self.as_path())
    }

    fn extension(&self) -> Option<&ffi::OsStr> {
        Path::extension(self.as_path())
    }

    fn components(&self) -> Components<'_> {
        Path::components(self.as_path())
    }

    fn iter(&self) -> path::Iter<'_> {
        Path::iter(self.as_path())
    }

    fn display(&self) -> path::Display<'_> {
        Path::display(self.as_path())
    }

    /// Queries the file system to get information about a file, directory, etc.
    fn metadata(&self) -> Result<fs::Metadata> {
        Path::metadata(self.as_path()).map_err(|err| {
            Error::new(err, "getting metadata of", self.to_arc_pathbuf())
        })
    }

    /// Queries the metadata about a file without following symlinks.
    fn symlink_metadata(&self) -> Result<fs::Metadata> {
        Path::symlink_metadata(self.as_path()).map_err(|err| {
            Error::new(
                err,
                "getting symlink metadata of",
                self.to_arc_pathbuf(),
            )
        })
    }

    fn exists(&self) -> bool {
        Path::exists(self.as_path())
    }

    fn is_file(&self) -> bool {
        Path::is_file(self.as_path())
    }

    fn is_dir(&self) -> bool {
        Path::is_dir(self.as_path())
    }

    /// Reads a symbolic link, returning the path that the link points to.
    fn read_link(&self) -> Result<PathBuf> {
        Path::read_link(self.as_path()).map_err(|err| {
            Error::new(err, "reading link target of", self.to_arc_pathbuf())
        })
    }

    /// Returns the canonical, absolute form of the path with all intermediate
    /// components normalized and symbolic links resolved.
    /// 返回路径的规范、绝对形式，其中所有中间组件均已标准化并已解析符号链接。
    fn canonicalize(&self) -> Result<PathAbs> {
        Path::canonicalize(self.as_path())
            .map(|path| PathAbs(path.into()))
            .map_err(|err| {
                Error::new(err, "canonicalizing", self.to_arc_pathbuf())
            })
    }

    /// Returns the path without its final component, if there is one.
    fn parent(&self) -> Result<&Path> {
        let parent_path = Path::parent(self.as_path());
        if let Some(p) = parent_path {
            Ok(p)
        } else {
            Err(Error::new(
                io::Error::new(io::ErrorKind::NotFound, "path has no parent"),
                "truncating to parent",
                self.to_arc_pathbuf(),
            ))
        }
    }
}

impl PathInfo for Path {
    fn as_path(&self) -> &Path {
        &self
    }
    fn to_arc_pathbuf(&self) -> Arc<PathBuf> {
        self.to_path_buf().into()
    }
}

impl<T> PathInfo for T
where
    T: Clone + Borrow<PathBuf> + Into<Arc<PathBuf>>,
{
    fn as_path(&self) -> &Path {
        // T 类型实现了 Borrow，可以通过方法borrow()将其转换为 &PathBuf 类型
        PathBuf::as_path(self.borrow())
    }
    fn to_arc_pathbuf(&self) -> Arc<PathBuf> {
        self.clone().into()
    }
}

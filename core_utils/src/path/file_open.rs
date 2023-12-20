use std::borrow::Borrow;
use std::fmt;
use std::fs;
use std::path::Path;

use super::{Error, PathAbs, PathFile, Result};

/// **INTERNAL TYPE: do not use directly.**
///
/// Use `FileRead`, `FileWrite` or `FileEdit` instead.
pub struct FileOpen {
    pub(crate) path: PathFile,
    pub(crate) file: fs::File,
}

impl FileOpen {
    /// Open the file with the given `OpenOptions`.
    pub fn open<P: AsRef<Path>>(
        path: P,
        options: fs::OpenOptions,
    ) -> Result<FileOpen> {
        // 打开文件
        let file = options.open(&path).map_err(|err| {
            Error::new(err, "opening", path.as_ref().to_path_buf().into())
        })?;

        let path = PathFile::new(path)?;
        Ok(FileOpen { path: path, file })
    }

    pub fn open_abs<P: Into<PathAbs>>(
        path: P,
        options: fs::OpenOptions,
    ) -> Result<FileOpen> {
        let path = path.into();
        // 打开文件
        let file = options
            .open(&path)
            .map_err(|err| Error::new(err, "opening", path.clone().into()))?;

        Ok(FileOpen { path: PathFile::new_unchecked(path), file })
    }

    /// Get the path associated with the open file.
    pub fn path(&self) -> &PathFile {
        &self.path
    }

    pub fn metadata(&self) -> Result<fs::Metadata> {
        self.file.metadata().map_err(|err| {
            Error::new(err, "getting metadata for", self.path.clone().into())
        })
    }

    pub fn try_clone(&self) -> Result<FileOpen> {
        let file = self.file.try_clone().map_err(|err| {
            Error::new(
                err,
                "cloning file handle for",
                self.path.clone().into(),
            )
        })?;
        Ok(FileOpen { file, path: self.path.clone() })
    }
}

impl fmt::Debug for FileOpen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Open(")?;
        self.path.fmt(f)?;
        write!(f, ")")
    }
}

impl AsRef<fs::File> for FileOpen {
    fn as_ref(&self) -> &fs::File {
        &self.file
    }
}

impl Borrow<fs::File> for FileOpen {
    fn borrow(&self) -> &fs::File {
        &self.file
    }
}

impl<'a> Borrow<fs::File> for &'a FileOpen {
    fn borrow(&self) -> &fs::File {
        &self.file
    }
}

impl From<FileOpen> for fs::File {
    fn from(orig: FileOpen) -> fs::File {
        orig.file
    }
}

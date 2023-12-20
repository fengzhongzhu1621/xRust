use std::borrow::Borrow;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use super::FileOpen;
use super::{Error, PathAbs, PathFile, PathInfo, Result};

pub struct FileWrite(pub(crate) FileOpen);

impl FileWrite {
    /// Open the file with the given `OpenOptions` but always sets `write` to true.
    pub fn open<P: AsRef<Path>>(
        path: P,
        mut options: fs::OpenOptions,
    ) -> Result<FileWrite> {
        options.write(true);
        Ok(FileWrite(FileOpen::open(path, options)?))
    }

    /// Shortcut to open the file if the path is already absolute.
    pub(crate) fn open_abs<P: Into<PathAbs>>(
        path: P,
        mut options: fs::OpenOptions,
    ) -> Result<FileWrite> {
        options.write(true);
        Ok(FileWrite(FileOpen::open_abs(path, options)?))
    }

    /// Open the file in write-only mode, truncating it first if it exists and creating it
    /// otherwise.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<FileWrite> {
        let mut options = fs::OpenOptions::new();
        options.truncate(true);
        options.create(true);
        FileWrite::open(path, options)
    }

    /// Open the file for appending, creating it if it doesn't exist.
    pub fn open_append<P: AsRef<Path>>(path: P) -> Result<FileWrite> {
        let mut options = fs::OpenOptions::new();
        options.append(true);
        options.create(true);
        FileWrite::open(path, options)
    }

    /// Open the file for editing (reading and writing) but do not create it
    /// if it doesn't exist.
    pub fn open_edit<P: AsRef<Path>>(path: P) -> Result<FileWrite> {
        let mut options = fs::OpenOptions::new();
        options.read(true);
        FileWrite::open(path, options)
    }

    pub fn path(&self) -> &PathFile {
        &self.0.path
    }

    pub fn sync_all(&self) -> Result<()> {
        self.0.file.sync_all().map_err(|err| {
            Error::new(err, "syncing", self.0.path.clone().into())
        })
    }

    pub fn sync_data(&self) -> Result<()> {
        self.0.file.sync_data().map_err(|err| {
            Error::new(err, "syncing data for", self.0.path.clone().into())
        })
    }

    pub fn set_len(&mut self, size: u64) -> Result<()> {
        self.0.file.set_len(size).map_err(|err| {
            Error::new(err, "setting len for", self.0.path.clone().into())
        })
    }

    pub fn set_permissions(&mut self, perm: fs::Permissions) -> Result<()> {
        self.0.file.set_permissions(perm).map_err(|err| {
            Error::new(
                err,
                "setting permisions for",
                self.0.path.clone().into(),
            )
        })
    }

    pub fn write_str(&mut self, s: &str) -> Result<()> {
        self.0.file.write_all(s.as_bytes()).map_err(|err| {
            Error::new(err, "writing", self.0.path.clone().into())
        })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.0.file.flush().map_err(|err| {
            Error::new(err, "flushing", self.0.path.clone().into())
        })
    }
}

impl fmt::Debug for FileWrite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileWrite(")?;
        self.0.path.fmt(f)?;
        write!(f, ")")
    }
}

impl io::Write for FileWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.file.write(buf).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when writing to {}", err, self.path().display()),
            )
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.file.flush().map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when flushing {}", err, self.path().display()),
            )
        })
    }
}

impl io::Seek for FileWrite {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.0.file.seek(pos).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} seeking {}", err, self.path().display()),
            )
        })
    }
}

impl AsRef<FileOpen> for FileWrite {
    fn as_ref(&self) -> &FileOpen {
        &self.0
    }
}

impl AsRef<File> for FileWrite {
    fn as_ref(&self) -> &File {
        self.0.as_ref()
    }
}

impl Borrow<FileOpen> for FileWrite {
    fn borrow(&self) -> &FileOpen {
        &self.0
    }
}

impl Borrow<File> for FileWrite {
    fn borrow(&self) -> &File {
        self.0.borrow()
    }
}

impl<'a> Borrow<FileOpen> for &'a FileWrite {
    fn borrow(&self) -> &FileOpen {
        &self.0
    }
}

impl<'a> Borrow<File> for &'a FileWrite {
    fn borrow(&self) -> &File {
        self.0.borrow()
    }
}

impl From<FileWrite> for FileOpen {
    fn from(orig: FileWrite) -> FileOpen {
        orig.0
    }
}

impl From<FileWrite> for File {
    fn from(orig: FileWrite) -> File {
        orig.0.into()
    }
}

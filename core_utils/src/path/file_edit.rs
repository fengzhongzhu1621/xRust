use super::FileOpen;
use super::{Error, PathAbs, PathInfo, Result};
use std::borrow::Borrow;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct FileEdit(pub(crate) FileOpen);

impl FileEdit {
    /// Open the file with the given `OpenOptions` but always sets `read` and `write` to true.
    pub fn open<P: AsRef<Path>>(
        path: P,
        mut options: fs::OpenOptions,
    ) -> Result<FileEdit> {
        options.write(true);
        options.read(true);
        Ok(FileEdit(FileOpen::open(path, options)?))
    }

    /// Shortcut to open the file if the path is already absolute.
    pub(crate) fn open_abs<P: Into<PathAbs>>(
        path: P,
        mut options: fs::OpenOptions,
    ) -> Result<FileEdit> {
        options.write(true);
        options.read(true);
        Ok(FileEdit(FileOpen::open_abs(path, options)?))
    }

    /// Open the file in editing mode, truncating it first if it exists and creating it
    /// otherwise.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<FileEdit> {
        let mut options = fs::OpenOptions::new();
        options.truncate(true);
        options.create(true);
        FileEdit::open(path, options)
    }

    /// Open the file for appending, creating it if it doesn't exist.
    pub fn append<P: AsRef<Path>>(path: P) -> Result<FileEdit> {
        let mut options = fs::OpenOptions::new();
        options.append(true);
        options.create(true);
        FileEdit::open(path, options)
    }

    /// Open the file for editing (reading and writing) but do not create it
    /// if it doesn't exist.
    pub fn edit<P: AsRef<Path>>(path: P) -> Result<FileEdit> {
        let mut options = fs::OpenOptions::new();
        options.read(true);
        FileEdit::open(path, options)
    }

    /// Attempts to sync all OS-internal metadata to disk.
    pub fn sync_all(&self) -> Result<()> {
        self.0.file.sync_all().map_err(|err| {
            Error::new(err, "syncing", self.0.path.clone().into())
        })
    }

    /// This function is similar to sync_all, except that it may not synchronize file metadata to
    /// the filesystem.
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

    /// Read what remains of the file to a `String`.
    pub fn read_string(&mut self) -> Result<String> {
        let mut s = String::new();
        self.0.file.read_to_string(&mut s).map_err(|err| {
            Error::new(err, "reading", self.0.path.clone().into())
        })?;
        Ok(s)
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

impl fmt::Debug for FileEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileEdit(")?;
        self.0.path.fmt(f)?;
        write!(f, ")")
    }
}

impl io::Read for FileEdit {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.file.read(buf).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when reading {}", err, self.0.path().display()),
            )
        })
    }
}

impl io::Write for FileEdit {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.file.write(buf).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when writing to {}", err, self.0.path().display()),
            )
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.file.flush().map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} when flushing {}", err, self.0.path().display()),
            )
        })
    }
}

impl io::Seek for FileEdit {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.0.file.seek(pos).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("{} seeking {}", err, self.0.path().display()),
            )
        })
    }
}

impl AsRef<FileOpen> for FileEdit {
    fn as_ref(&self) -> &FileOpen {
        &self.0
    }
}

impl AsRef<File> for FileEdit {
    fn as_ref(&self) -> &File {
        &self.0.as_ref()
    }
}

impl Borrow<FileOpen> for FileEdit {
    fn borrow(&self) -> &FileOpen {
        &self.0
    }
}

impl Borrow<File> for FileEdit {
    fn borrow(&self) -> &File {
        &self.0.borrow()
    }
}

impl<'a> Borrow<FileOpen> for &'a FileEdit {
    fn borrow(&self) -> &FileOpen {
        &self.0
    }
}

impl<'a> Borrow<File> for &'a FileEdit {
    fn borrow(&self) -> &File {
        &self.0.borrow()
    }
}

impl From<FileEdit> for FileOpen {
    fn from(orig: FileEdit) -> FileOpen {
        orig.0
    }
}

impl From<FileEdit> for File {
    fn from(orig: FileEdit) -> File {
        orig.0.into()
    }
}

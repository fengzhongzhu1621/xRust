use super::dir::{self, TempDir};
use super::temp_file::{self, NamedTempFile};
use super::temp_path::TempPath;
use super::util;

use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::path::Path;
use std::{env, io};

pub const NUM_RAND_CHARS: usize = 6;

/// Create a new temporary file or directory with custom parameters.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Builder<'a, 'b> {
    random_len: usize,
    prefix: &'a OsStr,
    suffix: &'b OsStr,
    append: bool,
}

impl<'a, 'b> Default for Builder<'a, 'b> {
    fn default() -> Self {
        Builder {
            random_len: NUM_RAND_CHARS,
            prefix: OsStr::new(".tmp"),
            suffix: OsStr::new(""),
            append: false,
        }
    }
}

impl<'a, 'b> Builder<'a, 'b> {
    /// Create a new `Builder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix<S: AsRef<OsStr> + ?Sized>(
        &mut self,
        prefix: &'a S,
    ) -> &mut Self {
        self.prefix = prefix.as_ref();
        self
    }

    pub fn suffix<S: AsRef<OsStr> + ?Sized>(
        &mut self,
        suffix: &'b S,
    ) -> &mut Self {
        self.suffix = suffix.as_ref();
        self
    }

    pub fn rand_bytes(&mut self, rand: usize) -> &mut Self {
        self.random_len = rand;
        self
    }

    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    /// Create the named temporary file.
    pub fn tempfile(&self) -> io::Result<NamedTempFile> {
        self.tempfile_in(env::temp_dir())
    }

    /// Create the named temporary file in the specified directory.
    pub fn tempfile_in<P: AsRef<Path>>(
        &self,
        dir: P,
    ) -> io::Result<NamedTempFile> {
        util::create_helper(
            dir.as_ref(),
            self.prefix,
            self.suffix,
            self.random_len,
            |path| {
                temp_file::create_named(
                    path,
                    OpenOptions::new().append(self.append),
                )
            },
        )
    }

    pub fn tempdir(&self) -> io::Result<TempDir> {
        self.tempdir_in(env::temp_dir())
    }

    pub fn tempdir_in<P: AsRef<Path>>(&self, dir: P) -> io::Result<TempDir> {
        let storage;
        let mut dir = dir.as_ref();
        if !dir.is_absolute() {
            let cur_dir = env::current_dir()?;
            storage = cur_dir.join(dir);
            dir = &storage;
        }

        util::create_helper(
            dir,
            self.prefix,
            self.suffix,
            self.random_len,
            dir::create,
        )
    }

    pub fn make<F, R>(&self, f: F) -> io::Result<NamedTempFile<R>>
    where
        F: FnMut(&Path) -> io::Result<R>,
    {
        self.make_in(env::temp_dir(), f)
    }

    pub fn make_in<F, R, P>(
        &self,
        dir: P,
        mut f: F,
    ) -> io::Result<NamedTempFile<R>>
    where
        F: FnMut(&Path) -> io::Result<R>,
        P: AsRef<Path>,
    {
        util::create_helper(
            dir.as_ref(),
            self.prefix,
            self.suffix,
            self.random_len,
            move |path| {
                Ok(NamedTempFile::from_parts(
                    f(&path)?,
                    TempPath::from_path(path),
                ))
            },
        )
    }
}

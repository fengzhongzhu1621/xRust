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
                file::create_named(
                    path,
                    OpenOptions::new().append(self.append),
                )
            },
        )
    }
}

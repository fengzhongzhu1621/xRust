use super::{Error, PathInfo, Result};
use std::ffi;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

pub trait PathMut: PathInfo {
    /// Appends `path` to this path.
    /// P通过方法as_ref()实现了到 &Path的转换
    fn append<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;

    /// Go "up" one directory.
    fn pop_up(&mut self) -> Result<()>;

    /// Removes all components after the root, if any.
    fn truncate_to_root(&mut self);

    fn set_file_name<S: AsRef<ffi::OsStr>>(&mut self, file_name: S);

    fn set_extension<S: AsRef<ffi::OsStr>>(&mut self, extension: S) -> bool;
}

impl PathMut for PathBuf {
    fn append<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        for each in path.as_ref().components() {
            match each {
                Component::Normal(c) => self.push(c),
                Component::CurDir => (), // "." does nothing
                Component::Prefix(_) => {
                    return Err(Error::new(
                        io::Error::new(
                            io::ErrorKind::Other,
                            "appended path has a prefix",
                        ),
                        "appending path",
                        path.as_ref().to_path_buf().into(),
                    ));
                }
                Component::RootDir => (), // leading "/" does nothing
                Component::ParentDir => self.pop_up()?,
            }
        }

        Ok(())
    }

    fn pop_up(&mut self) -> Result<()> {
        /// Pop off the parent components and return how
        /// many were removed.
        fn pop_parent_components(p: &mut PathBuf) -> usize {
            let mut cur_dirs: usize = 0;
            let mut parents: usize = 0;
            let mut components = p.components();
            while let Some(c) = components.next_back() {
                match c {
                    Component::CurDir => cur_dirs += 1,
                    Component::ParentDir => parents += 1,
                    _ => break,
                }
            }
            for _ in 0..(cur_dirs + parents) {
                p.pop();
            }
            parents
        }

        let mut ending_parents = 0;
        loop {
            ending_parents += pop_parent_components(self);
            if ending_parents == 0 || self.file_name().is_none() {
                break;
            } else {
                // we have at least one "parent" to consume
                self.pop();
                ending_parents -= 1;
            }
        }

        if self.pop() {
            // do nothing, success
        } else if self.has_root() {
            // We tried to pop off the root
            return Err(Error::new(
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "cannot get parent of root path",
                ),
                "truncating to parent",
                self.clone().into(),
            ));
        } else {
            // we are creating a relative path, `"../"`
            self.push("..")
        }

        // Put all unhandled parents back, creating a relative path.
        for _ in 0..ending_parents {
            self.push("..")
        }

        Ok(())
    }

    fn truncate_to_root(&mut self) {
        let mut res = PathBuf::new();
        for component in self.components().take(2) {
            match component {
                // We want to keep prefix and RootDir components of this path
                Component::Prefix(_) | Component::RootDir => {
                    res.push(component)
                }
                // We want to discard all other components.
                _ => break,
            }
        }

        // Clobber ourselves with the new value.
        *self = res;
    }

    fn set_file_name<S: AsRef<ffi::OsStr>>(&mut self, file_name: S) {
        self.set_file_name(file_name)
    }

    fn set_extension<S: AsRef<ffi::OsStr>>(&mut self, extension: S) -> bool {
        self.set_extension(extension)
    }
}

impl PathMut for Arc<PathBuf> {
    fn append<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Arc::make_mut(self).append(path)
    }
    fn pop_up(&mut self) -> Result<()> {
        Arc::make_mut(self).pop_up()
    }
    fn truncate_to_root(&mut self) {
        Arc::make_mut(self).truncate_to_root()
    }
    fn set_file_name<S: AsRef<ffi::OsStr>>(&mut self, file_name: S) {
        Arc::make_mut(self).set_file_name(file_name)
    }
    fn set_extension<S: AsRef<ffi::OsStr>>(&mut self, extension: S) -> bool {
        Arc::make_mut(self).set_extension(extension)
    }
}

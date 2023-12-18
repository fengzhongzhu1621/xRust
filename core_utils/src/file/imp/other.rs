use crate::error::not_supported;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub fn create_named(
    _path: &Path,
    _open_options: &mut OpenOptions,
) -> io::Result<File> {
    not_supported()
}

pub fn create(_dir: &Path) -> io::Result<File> {
    not_supported()
}

pub fn reopen(_file: &File, _path: &Path) -> io::Result<File> {
    not_supported()
}

pub fn persist(
    _old_path: &Path,
    _new_path: &Path,
    _overwrite: bool,
) -> io::Result<()> {
    not_supported()
}

pub fn keep(_path: &Path) -> io::Result<()> {
    not_supported()
}

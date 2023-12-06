/// Create a new temporary file.
use super::imp;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::{env, io};

pub fn tempfile() -> io::Result<File> {
    tempfile_in(env::temp_dir())
}

/// 创建临时文件
/// Create a new temporary file in the specified directory.
/// AsRef是一个用于实现引用转换的特型(trait)，AsRef<T>相当于&T。
pub fn tempfile_in<P: AsRef<Path>>(dir: P) -> io::Result<File> {
    imp::create(dir.as_ref())
}

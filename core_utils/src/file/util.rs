use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::{io, iter::repeat_with};

use crate::error::IoResultExt;

pub const NUM_RAND_CHARS: usize = 6;

const NUM_RETRIES: u32 = 1 << 31; // 2147483648

/// 构造临时文件的名称
fn tmpname(prefix: &OsStr, suffix: &OsStr, rand_len: usize) -> OsString {
    let mut buf =
        OsString::with_capacity(prefix.len() + suffix.len() + rand_len);
    buf.push(prefix);
    let mut char_buf = [0u8; 4];
    // 创建一个新的迭代器，通过应用提供的闭包，重复器 F: FnMut() -> A 来无限地重复类型为 A 的元素。
    //
    for c in repeat_with(fastrand::alphanumeric).take(rand_len) {
        // encode_utf8 将一个字符转成 u8 数组
        buf.push(c.encode_utf8(&mut char_buf));
    }
    buf.push(suffix);
    buf
}

/// 创建临时文件，支持重试直到成功为止
pub fn create_helper<R>(
    base: &Path,
    prefix: &OsStr,
    suffix: &OsStr,
    random_len: usize,
    mut f: impl FnMut(PathBuf) -> io::Result<R>,
) -> io::Result<R> {
    // 重试次数
    let num_retries = if random_len != 0 { NUM_RETRIES } else { 1 };

    for _ in 0..num_retries {
        // 构造临时文件的路径
        let file_path = base.join(tmpname(prefix, suffix, random_len));
        // 创建临时文件
        return match f(file_path) {
            Err(ref e)
                if e.kind() == io::ErrorKind::AlreadyExists
                    && num_retries > 1 =>
            {
                continue
            }
            // AddrInUse can happen if we're creating a UNIX domain socket and
            // the path already exists.
            Err(ref e)
                if e.kind() == io::ErrorKind::AddrInUse && num_retries > 1 =>
            {
                continue
            }
            res => res,
        };
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "too many temporary files exist",
    ))
    .with_err_path(|| base)
}

//! This module implements formatting functions for writing into lock files.

use crate::platform::sys;
use core::{
    fmt::{self, Write},
    mem,
};

/// I/O buffer size, chosen targeting possible PID's digits (I belive 11 would
/// be enough tho).
const BUF_SIZE: usize = 16;

/// A fmt Writer that writes data into the given open file.
///
/// # Example
///
/// writeln!(fmt::Writer(self.desc), "{}", sys::pid())
#[derive(Debug, Clone, Copy)]
pub struct Writer(
    /// The open file to which data will be written.
    pub sys::FileDesc,
);

impl Writer {
    /// Writes formatting arguments into the file.
    pub fn write_fmt(
        &self,
        arguments: fmt::Arguments,
    ) -> Result<(), sys::Error> {
        let mut adapter = Adapter::new(self.0);
        let _ = adapter.write_fmt(arguments);
        adapter.finish()
    }
}

/// Fmt <-> IO adapter.
///
/// Buffer is flushed on drop.
#[derive(Debug)]
struct Adapter {
    /// File being written to.
    desc: sys::FileDesc,
    /// Temporary buffer of bytes being written.
    buffer: [u8; BUF_SIZE],
    /// Cursor tracking where new bytes should be written at the buffer.
    cursor: usize,
    /// Partial result for writes.
    result: Result<(), sys::Error>,
}

/// 自定义写缓存
impl Adapter {
    /// Creates a zeroed adapter from an open file.
    fn new(desc: sys::FileDesc) -> Self {
        Self { desc, buffer: [0; BUF_SIZE], cursor: 0, result: Ok(()) }
    }

    /// Flushes the buffer into the open file.
    fn flush(&mut self) -> Result<(), sys::Error> {
        // 将缓存结果写入到文件，并清空缓存
        sys::write(self.desc, &self.buffer[..self.cursor])?;
        self.buffer = [0; BUF_SIZE];
        self.cursor = 0;
        Ok(())
    }

    /// Finishes the adapter, returning the I/O Result
    fn finish(mut self) -> Result<(), sys::Error> {
        // 将 self.result复制为 Ok(())
        mem::replace(&mut self.result, Ok(()))
    }
}

impl Write for Adapter {
    /// 写文件，失败一次后，后续写操作无效
    fn write_str(&mut self, data: &str) -> fmt::Result {
        let mut bytes = data.as_bytes();

        // 写入前先判断上一次刷新操作是否写成功，如果失败则不会重新写入，除非调用了finish()将其标记为可用
        while bytes.len() > 0 && self.result.is_ok() {
            let start = self.cursor;
            // 计算缓存可用大小
            let size = (BUF_SIZE - self.cursor).min(bytes.len());
            let end = start + size;
            // 复制到缓存
            self.buffer[start..end].copy_from_slice(&bytes[..size]);
            self.cursor = end;

            // 获得还未放入缓存的数据
            bytes = &bytes[size..];

            if bytes.len() > 0 {
                // 缓存满则将缓存结果写入到文件，并清空缓存，返回的结果表明是否写入成功
                self.result = self.flush();
            }
        }

        match self.result {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        // 将缓存结果写入到文件，并清空缓存
        let _ = self.flush();
        // 写入到磁盘
        let _ = sys::fsync(self.desc);
    }
}

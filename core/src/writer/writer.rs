use std::{io, sync::Mutex};
use crate::writer::{Buffer, WriteStyle};


/// 定义输出目标
pub enum WritableTarget {
    /// Logs will be sent to standard output.
    Stdout,
    /// Logs will be sent to standard error.
    Stderr,
    /// Logs will be sent to a custom pipe.
    Pipe(Box<Mutex<dyn io::Write + Send + 'static>>),
}

/// 写 buffer，包含一个写数据的目标
pub struct BufferWriter {
    target: WritableTarget,
}

/// BufferWriter 构造函数
impl BufferWriter {
    pub fn stderr(_is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter {
            target: WritableTarget::Stderr,
        }
    }

    pub fn stdout(_is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter {
            target: WritableTarget::Stdout,
        }
    }

    pub fn pipe(
        _write_style: WriteStyle,
        pipe: Box<Mutex<dyn io::Write + Send + 'static>>,
    ) -> Self {
        BufferWriter {
            target: WritableTarget::Pipe(pipe),
        }
    }
}


/// BufferWriter 输出数据
impl BufferWriter {
    pub fn buffer(&self) -> Buffer {
        Buffer(Vec::new())
    }

    /// 输出到指定目标
    pub fn print(&self, buf: &Buffer) -> io::Result<()> {
        // This impl uses the `eprint` and `print` macros
        // instead of using the streams directly.
        // This is so their output can be captured by `cargo test`.
        match &self.target {
            // Safety: If the target type is `Pipe`, `target_pipe` will always be non-empty.
            // 支持并发写数据
            WritableTarget::Pipe(pipe) => pipe.lock().unwrap().write_all(&buf.0)?,
            // 将字节切片转换为字符串，包括无效字符
            WritableTarget::Stdout => print!("{}", String::from_utf8_lossy(&buf.0)),
            WritableTarget::Stderr => eprint!("{}", String::from_utf8_lossy(&buf.0)),
        }

        Ok(())
    }
}

/// A terminal target with color awareness.
pub struct Writer {
    inner: BufferWriter,
    write_style: WriteStyle,
}

impl Writer {
    pub fn write_style(&self) -> WriteStyle {
        self.write_style
    }

    pub fn buffer(&self) -> Buffer {
        self.inner.buffer()
    }

    pub fn print(&self, buf: &Buffer) -> io::Result<()> {
        self.inner.print(buf)
    }
}

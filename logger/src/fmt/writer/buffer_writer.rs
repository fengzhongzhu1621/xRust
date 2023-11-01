use crate::fmt::writer::{Buffer, WritableTarget, WriteStyle};
use std::io;
use std::sync::Mutex;

pub struct BufferWriter {
    target: WritableTarget, // 指定输出目标
}

/// BufferWriter 构造函数
impl BufferWriter {
    pub fn stderr(_is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter { target: WritableTarget::Stderr }
    }

    pub fn stdout(_is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter { target: WritableTarget::Stdout }
    }

    pub fn pipe(
        _write_style: WriteStyle,
        pipe: Box<Mutex<dyn io::Write + Send + 'static>>,
    ) -> Self {
        BufferWriter { target: WritableTarget::Pipe(pipe) }
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
            WritableTarget::Pipe(pipe) => {
                pipe.lock().unwrap().write_all(&buf.0)?
            }
            // 将字节切片转换为字符串，包括无效字符
            WritableTarget::Stdout => {
                print!("{}", String::from_utf8_lossy(&buf.0))
            }
            WritableTarget::Stderr => {
                eprint!("{}", String::from_utf8_lossy(&buf.0))
            }
        }

        Ok(())
    }
}

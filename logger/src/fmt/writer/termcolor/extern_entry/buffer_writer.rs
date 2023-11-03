use super::Buffer;
use crate::fmt::writer::{WritableTarget, WriteStyle};
use std::io;
use std::sync::Mutex;
use termcolor;

/// 对 termcolor 内置的 BufferWriter 进行功能扩展
pub struct BufferWriter {
    pub inner: termcolor::BufferWriter,
    pub uncolored_target: Option<WritableTarget>,
}

impl BufferWriter {
    pub fn stderr(is_test: bool, write_style: WriteStyle) -> Self {
        BufferWriter {
            inner: termcolor::BufferWriter::stderr(write_style.into_color_choice()),
            uncolored_target: if is_test {
                Some(WritableTarget::Stderr)
            } else {
                None
            },
        }
    }

    pub fn stdout(is_test: bool, write_style: WriteStyle) -> Self {
        BufferWriter {
            inner: termcolor::BufferWriter::stdout(write_style.into_color_choice()),
            uncolored_target: if is_test {
                Some(WritableTarget::Stdout)
            } else {
                None
            },
        }
    }

    pub fn pipe(
        write_style: WriteStyle,
        pipe: Box<Mutex<dyn io::Write + Send + 'static>>,
    ) -> Self {
        BufferWriter {
            // The inner Buffer is never printed from, but it is still needed to handle coloring and other formatting
            inner: termcolor::BufferWriter::stderr(write_style.into_color_choice()),
            uncolored_target: Some(WritableTarget::Pipe(pipe)),
        }
    }

    /// 创建默认缓存
    pub fn buffer(&self) -> Buffer {
        Buffer {
            inner: self.inner.buffer(),
            has_uncolored_target: self.uncolored_target.is_some(),
        }
    }

    pub fn print(&self, buf: &Buffer) -> io::Result<()> {
        if let Some(target) = &self.uncolored_target {
            // This impl uses the `eprint` and `print` macros
            // instead of `termcolor`'s buffer.
            // This is so their output can be captured by `cargo test`
            let log = String::from_utf8_lossy(buf.bytes());

            match target {
                WritableTarget::Stderr => eprint!("{}", log),
                WritableTarget::Stdout => print!("{}", log),
                WritableTarget::Pipe(pipe) => {
                    write!(pipe.lock().unwrap(), "{}", log)?
                }
            }

            Ok(())
        } else {
            self.inner.print(&buf.inner)
        }
    }
}

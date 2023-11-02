use super::Buffer;
use crate::fmt::writer::{WritableTarget, WriteStyle};
use std::io;
use std::sync::Mutex;
use termcolor::{self, ColorChoice, ColorSpec, WriteColor};

pub struct BufferWriter {
    pub inner: termcolor::BufferWriter,
    pub uncolored_target: Option<WritableTarget>,
}

impl BufferWriter {
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

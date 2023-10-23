use std::{io, sync::Mutex};

use crate::fmt::{WritableTarget, WriteStyle};

pub struct Buffer(Vec<u8>);

impl Buffer {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend(buf);
        Ok(buf.len())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[cfg(test)]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

pub struct BufferWriter {
    target: WritableTarget,
}

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

    pub fn buffer(&self) -> Buffer {
        Buffer(Vec::new())
    }

    pub fn print(&self, buf: &Buffer) -> io::Result<()> {
        // This impl uses the `eprint` and `print` macros
        // instead of using the streams directly.
        // This is so their output can be captured by `cargo test`.
        match &self.target {
            // Safety: If the target type is `Pipe`, `target_pipe` will always be non-empty.
            WritableTarget::Pipe(pipe) => {
                pipe.lock().unwrap().write_all(&buf.0)?
            }
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

use crate::fmt::writer::{Buffer, BufferWriter, WriteStyle};
use std::io;

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

use crate::fmt::writer::{Buffer, BufferWriter, WriteStyle};
use std::fmt;
use std::io;

/// A terminal target with color awareness.
pub struct Writer {
    pub inner: BufferWriter,
    pub write_style: WriteStyle,
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

impl fmt::Debug for Writer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Writer").finish()
    }
}

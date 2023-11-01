use std::io;
use std::io::Write;
use termcolor::{self, ColorSpec, WriteColor};

pub struct Buffer {
    inner: termcolor::Buffer,
    has_uncolored_target: bool,
}

impl Buffer {
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    pub fn bytes(&self) -> &[u8] {
        self.inner.as_slice()
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        // Ignore styles for test captured logs because they can't be printed
        if !self.has_uncolored_target {
            self.inner.set_color(spec)
        } else {
            Ok(())
        }
    }

    fn reset(&mut self) -> io::Result<()> {
        // Ignore styles for test captured logs because they can't be printed
        if !self.has_uncolored_target {
            self.inner.reset()
        } else {
            Ok(())
        }
    }
}

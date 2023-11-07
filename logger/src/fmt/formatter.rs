use crate::fmt::writer::WriteStyle;
use crate::fmt::writer::{Buffer, Writer};
use log::Record;
use std::cell::RefCell;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;

pub struct Formatter {
    pub buf: Rc<RefCell<Buffer>>,
    pub write_style: WriteStyle,
}

impl Formatter {
    pub fn new(writer: &Writer) -> Self {
        Formatter {
            buf: Rc::new(RefCell::new(writer.buffer())),
            write_style: writer.write_style(),
        }
    }

    pub fn write_style(&self) -> WriteStyle {
        self.write_style
    }

    pub fn print(&self, writer: &Writer) -> io::Result<()> {
        writer.print(&self.buf.borrow())
    }

    pub fn clear(&mut self) {
        self.buf.borrow_mut().clear()
    }
}

impl Write for Formatter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.borrow_mut().flush()
    }
}

impl fmt::Debug for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Formatter").finish()
    }
}

pub type FormatFn =
    Box<dyn Fn(&mut Formatter, &Record) -> io::Result<()> + Sync + Send>;

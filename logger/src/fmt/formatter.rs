use crate::fmt::writer::WriteStyle;
use crate::fmt::writer::{Buffer, Writer};
use std::cell::RefCell;
use std::io;
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

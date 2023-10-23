use crate::fmt::{Buffer, WriteStyle};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Formatter {
    pub buf: Rc<RefCell<Buffer>>, // 运行时可写缓存
    write_style: WriteStyle,      // 日志输出的风格
}

/// 根据 color 特性加载 Buffer 和 BufferWriter
#[cfg_attr(feature = "color", path = "extern_impl.rs")]
#[cfg_attr(not(feature = "color"), path = "shim_impl.rs")]
mod imp;

pub(in crate::fmt) use self::imp::{Buffer, BufferWriter};

use std::{fmt, io};

/// Whether or not to print styles to the target.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WriteStyle {
    /// Try to print styles, but don't force the issue.
    Auto,
    /// Try very hard to print styles.
    Always,
    /// Never print styles.
    Never,
}

/// 设置 WriteStyle 的默认值
impl Default for WriteStyle {
    fn default() -> Self {
        WriteStyle::Auto
    }
}

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

/// 字符串转换为 WriteStyle 类型
pub fn parse_write_style(spec: &str) -> WriteStyle {
    match spec {
        "auto" => WriteStyle::Auto,
        "always" => WriteStyle::Always,
        "never" => WriteStyle::Never,
        _ => Default::default(),
    }
}

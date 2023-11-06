mod builder;
mod target;
mod termcolor;
mod write_style;
mod writer;
mod atty;

pub use builder::Builder;
pub use target::{Target, WritableTarget};
pub use termcolor::{Buffer, BufferWriter};
pub use write_style::{parse_write_style, WriteStyle};
pub use writer::Writer;

// 根据特性引入不同的方法
pub use atty::{is_stderr, is_stdout};

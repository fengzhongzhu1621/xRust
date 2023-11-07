mod atty;
// 用于构造 Writer
mod builder;
mod target;
pub mod termcolor;
mod write_style;
mod writer;

// 根据特性引入不同的方法
pub use atty::{is_stderr, is_stdout};

pub use builder::Builder as WriteBuilder;
pub use target::{Target, WritableTarget};
pub use termcolor::{Buffer, BufferWriter, SubtleStyle};
pub use write_style::{parse_write_style, WriteStyle};
pub use writer::Writer;

mod atty;
mod formatter;
mod writer;

// 根据特性引入不同的方法
pub use atty::{is_stderr, is_stdout};

pub use formatter::Formatter;
pub use writer::*;

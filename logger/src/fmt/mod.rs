mod atty;
mod builder;
mod formatter;
mod humantime;
mod target;
mod writer;

use atty::{is_stderr, is_stdout};
use builder::Builder;
use formatter::Formatter;
use target::{Target, WritableTarget};
use writer::{parse_write_style, Buffer, BufferWriter, WriteStyle, Writer};

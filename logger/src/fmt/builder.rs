use super::formatter::{FormatFn, Formatter};
use super::time::TimestampPrecision;
use crate::fmt::SubtleStyle;
use std::fmt::Display;
use std::io::prelude::*;

#[cfg(feature = "color")]
use crate::fmt::termcolor::Color;

use log::Record;
use std::{io, mem};

pub struct Builder {
    pub format_timestamp: Option<TimestampPrecision>,
    pub format_module_path: bool,
    pub format_target: bool,
    pub format_level: bool,
    pub format_indent: Option<usize>,
    pub custom_format: Option<FormatFn>,
    pub format_suffix: &'static str,
    built: bool,
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            format_timestamp: Some(Default::default()),
            format_module_path: false,
            format_target: true,
            format_level: true,
            format_indent: Some(4),
            custom_format: None,
            format_suffix: "\n",
            built: false,
        }
    }
}

impl Builder {
    /// Convert the format into a callable function.
    ///
    /// If the `custom_format` is `Some`, then any `default_format` switches are ignored.
    /// If the `custom_format` is `None`, then a default format is returned.
    /// Any `default_format` switches set to `false` won't be written by the format.
    pub fn build(&mut self) -> FormatFn {
        assert!(!self.built, "attempt to re-use consumed builder");

        let built =
            mem::replace(self, Builder { built: true, ..Default::default() });

        if let Some(fmt) = built.custom_format {
            fmt
        } else {
            Box::new(move |buf, record| {
                let fmt = DefaultFormat {
                    timestamp: built.format_timestamp,
                    module_path: built.format_module_path,
                    target: built.format_target,
                    level: built.format_level,
                    written_header_value: false,
                    indent: built.format_indent,
                    suffix: built.format_suffix,
                    buf,
                };

                fmt.write(record)
            })
        }
    }
}

/// The default format.
///
/// This format needs to work with any combination of crate features.
pub struct DefaultFormat<'a> {
    pub timestamp: Option<TimestampPrecision>,
    pub module_path: bool,
    pub target: bool,
    pub level: bool,
    pub written_header_value: bool,
    pub indent: Option<usize>,
    pub buf: &'a mut Formatter,
    pub suffix: &'a str,
}

impl<'a> DefaultFormat<'a> {
    pub fn write(mut self, record: &Record) -> io::Result<()> {
        self.write_timestamp()?;
        self.write_level(record)?;
        self.write_module_path(record)?;
        self.write_target(record)?;
        self.finish_header()?;

        self.write_args(record)
    }

    pub fn subtle_style(&self, text: &'static str) -> SubtleStyle {
        #[cfg(feature = "color")]
        {
            self.buf
                .style()
                .set_color(Color::Black)
                .set_intense(true)
                .clone()
                .into_value(text)
        }
        #[cfg(not(feature = "color"))]
        {
            text
        }
    }

    pub fn write_header_value<T>(&mut self, value: T) -> io::Result<()>
    where
        T: Display,
    {
        // 头部信息只写入一次
        if !self.written_header_value {
            self.written_header_value = true;

            // 获得只需要打印的 Style 类型
            let open_brace = self.subtle_style("[");
            write!(self.buf, "{}{}", open_brace, value)
        } else {
            write!(self.buf, " {}", value)
        }
    }

    pub fn write_level(&mut self, record: &Record) -> io::Result<()> {
        if !self.level {
            return Ok(());
        }

        let level = {
            #[cfg(feature = "color")]
            {
                self.buf.default_styled_level(record.level())
            }
            #[cfg(not(feature = "color"))]
            {
                record.level()
            }
        };

        self.write_header_value(format_args!("{:<5}", level))
    }

    pub fn write_timestamp(&mut self) -> io::Result<()> {
        #[cfg(feature = "humantime")]
        {
            use super::TimestampPrecision::*;
            let ts = match self.timestamp {
                None => return Ok(()),
                Some(Seconds) => self.buf.timestamp_seconds(),
                Some(Millis) => self.buf.timestamp_millis(),
                Some(Micros) => self.buf.timestamp_micros(),
                Some(Nanos) => self.buf.timestamp_nanos(),
            };

            self.write_header_value(ts)
        }
        #[cfg(not(feature = "humantime"))]
        {
            // Trick the compiler to think we have used self.timestamp
            // Workaround for "field is never used: `timestamp`" compiler nag.
            let _ = self.timestamp;
            Ok(())
        }
    }

    pub fn write_module_path(&mut self, record: &Record) -> io::Result<()> {
        if !self.module_path {
            return Ok(());
        }

        if let Some(module_path) = record.module_path() {
            self.write_header_value(module_path)
        } else {
            Ok(())
        }
    }

    pub fn write_target(&mut self, record: &Record) -> io::Result<()> {
        if !self.target {
            return Ok(());
        }

        match record.target() {
            "" => Ok(()),
            target => self.write_header_value(target),
        }
    }

    pub fn finish_header(&mut self) -> io::Result<()> {
        if self.written_header_value {
            let close_brace = self.subtle_style("]");
            write!(self.buf, "{} ", close_brace)
        } else {
            Ok(())
        }
    }

    pub fn write_args(&mut self, record: &Record) -> io::Result<()> {
        match self.indent {
            // Fast path for no indentation
            None => write!(self.buf, "{}{}", record.args(), self.suffix),

            Some(indent_count) => {
                // Create a wrapper around the buffer only if we have to actually indent the message

                struct IndentWrapper<'a, 'b: 'a> {
                    fmt: &'a mut DefaultFormat<'b>,
                    indent_count: usize,
                }

                impl<'a, 'b> Write for IndentWrapper<'a, 'b> {
                    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                        let mut first = true;
                        for chunk in buf.split(|&x| x == b'\n') {
                            if !first {
                                write!(
                                    self.fmt.buf,
                                    "{}{:width$}",
                                    self.fmt.suffix,
                                    "",
                                    width = self.indent_count
                                )?;
                            }
                            self.fmt.buf.write_all(chunk)?;
                            first = false;
                        }

                        Ok(buf.len())
                    }

                    fn flush(&mut self) -> io::Result<()> {
                        self.fmt.buf.flush()
                    }
                }

                // The explicit scope here is just to make older versions of Rust happy
                {
                    let mut wrapper =
                        IndentWrapper { fmt: self, indent_count };
                    write!(wrapper, "{}", record.args())?;
                }

                write!(self.buf, "{}", self.suffix)?;

                Ok(())
            }
        }
    }
}

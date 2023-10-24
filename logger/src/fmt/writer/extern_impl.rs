use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;
use std::sync::Mutex;

use log::Level;
use termcolor::{self, ColorChoice, ColorSpec, WriteColor};

use crate::fmt::{Formatter, WritableTarget, WriteStyle};

mod color;
use color::Color;

impl Formatter {
    /// 创建格式化风格
    pub fn style(&self) -> Style {
        Style { buf: self.buf.clone(), spec: ColorSpec::new() }
    }

    /// Get the default [`Style`] for the given level.
    ///
    /// The style can be used to print other values besides the level.
    pub fn default_level_style(&self, level: Level) -> Style {
        let mut level_style = self.style();
        match level {
            Level::Trace => level_style.set_color(Color::Cyan),
            Level::Debug => level_style.set_color(Color::Blue),
            Level::Info => level_style.set_color(Color::Green),
            Level::Warn => level_style.set_color(Color::Yellow),
            Level::Error => level_style.set_color(Color::Red).set_bold(true),
        };
        level_style
    }

    /// Get a printable [`Style`] for the given level.
    ///
    /// The style can only be used to print the level.
    pub fn default_styled_level(
        &self,
        level: Level,
    ) -> StyledValue<'static, Level> {
        self.default_level_style(level).into_value(level)
    }
}

pub struct BufferWriter {
    inner: termcolor::BufferWriter,
    uncolored_target: Option<WritableTarget>,
}

pub struct Buffer {
    inner: termcolor::Buffer,
    has_uncolored_target: bool,
}

impl BufferWriter {
    pub fn stderr(is_test: bool, write_style: WriteStyle) -> Self {
        BufferWriter {
            inner: termcolor::BufferWriter::stderr(
                write_style.into_color_choice(),
            ),
            uncolored_target: if is_test {
                Some(WritableTarget::Stderr)
            } else {
                None
            },
        }
    }

    pub fn stdout(is_test: bool, write_style: WriteStyle) -> Self {
        BufferWriter {
            inner: termcolor::BufferWriter::stdout(
                write_style.into_color_choice(),
            ),
            uncolored_target: if is_test {
                Some(WritableTarget::Stdout)
            } else {
                None
            },
        }
    }

    pub fn pipe(
        write_style: WriteStyle,
        pipe: Box<Mutex<dyn io::Write + Send + 'static>>,
    ) -> Self {
        BufferWriter {
            // The inner Buffer is never printed from, but it is still needed to handle coloring and other formatting
            inner: termcolor::BufferWriter::stderr(
                write_style.into_color_choice(),
            ),
            uncolored_target: Some(WritableTarget::Pipe(pipe)),
        }
    }

    pub fn buffer(&self) -> Buffer {
        Buffer {
            inner: self.inner.buffer(),
            has_uncolored_target: self.uncolored_target.is_some(),
        }
    }

    pub fn print(&self, buf: &Buffer) -> io::Result<()> {
        if let Some(target) = &self.uncolored_target {
            // This impl uses the `eprint` and `print` macros
            // instead of `termcolor`'s buffer.
            // This is so their output can be captured by `cargo test`
            let log = String::from_utf8_lossy(buf.bytes());

            match target {
                WritableTarget::Stderr => eprint!("{}", log),
                WritableTarget::Stdout => print!("{}", log),
                WritableTarget::Pipe(pipe) => {
                    write!(pipe.lock().unwrap(), "{}", log)?
                }
            }

            Ok(())
        } else {
            self.inner.print(&buf.inner)
        }
    }
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

impl WriteStyle {
    fn into_color_choice(self) -> ColorChoice {
        match self {
            WriteStyle::Always => ColorChoice::Always,
            WriteStyle::Auto => ColorChoice::Auto,
            WriteStyle::Never => ColorChoice::Never,
        }
    }
}

/// A set of styles to apply to the terminal output.
#[derive(Clone)]
pub struct Style {
    buf: Rc<RefCell<Buffer>>,
    spec: ColorSpec,
}

/// A value that can be printed using the given styles.
///
/// It is the result of calling [`Style::value`].
///
/// [`Style::value`]: struct.Style.html#method.value
pub struct StyledValue<'a, T> {
    style: Cow<'a, Style>,
    value: T,
}

impl Style {
    /// Set the text color.
    pub fn set_color(&mut self, color: Color) -> &mut Style {
        self.spec.set_fg(Some(color.into_termcolor()));
        self
    }

    /// Set the text weight.
    pub fn set_bold(&mut self, yes: bool) -> &mut Style {
        self.spec.set_bold(yes);
        self
    }

    /// Set the text intensity.
    pub fn set_intense(&mut self, yes: bool) -> &mut Style {
        self.spec.set_intense(yes);
        self
    }

    /// Set whether the text is dimmed.
    pub fn set_dimmed(&mut self, yes: bool) -> &mut Style {
        self.spec.set_dimmed(yes);
        self
    }

    /// Set the background color.
    pub fn set_bg(&mut self, color: Color) -> &mut Style {
        self.spec.set_bg(Some(color.into_termcolor()));
        self
    }

    /// Wrap a value in the style.
    pub fn value<T>(&self, value: T) -> StyledValue<T> {
        StyledValue { style: Cow::Borrowed(self), value }
    }

    /// Wrap a value in the style by taking ownership of it.
    pub(crate) fn into_value<T>(self, value: T) -> StyledValue<'static, T> {
        StyledValue { style: Cow::Owned(self), value }
    }
}

impl<'a, T> StyledValue<'a, T> {
    fn write_fmt<F>(&self, f: F) -> fmt::Result
    where
        F: FnOnce() -> fmt::Result,
    {
        self.style
            .buf
            .borrow_mut()
            .set_color(&self.style.spec)
            .map_err(|_| fmt::Error)?;

        // Always try to reset the terminal style, even if writing failed
        let write = f();
        let reset =
            self.style.buf.borrow_mut().reset().map_err(|_| fmt::Error);

        write.and(reset)
    }
}

impl fmt::Debug for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Style").field("spec", &self.spec).finish()
    }
}

macro_rules! impl_styled_value_fmt {
    ($($fmt_trait:path),*) => {
        $(
            impl<'a, T: $fmt_trait> $fmt_trait for StyledValue<'a, T> {
                fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
                    self.write_fmt(|| T::fmt(&self.value, f))
                }
            }
        )*
    };
}

impl_styled_value_fmt!(
    fmt::Debug,
    fmt::Display,
    fmt::Pointer,
    fmt::Octal,
    fmt::Binary,
    fmt::UpperHex,
    fmt::LowerHex,
    fmt::UpperExp,
    fmt::LowerExp
);

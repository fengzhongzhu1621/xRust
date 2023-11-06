use super::{Buffer, Color};
use crate::fmt::WriteStyle;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use termcolor::{ColorChoice, ColorSpec};

impl WriteStyle {
    pub fn into_color_choice(self) -> ColorChoice {
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

impl fmt::Debug for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Style").field("spec", &self.spec).finish()
    }
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
    pub fn into_value<T>(self, value: T) -> StyledValue<'static, T> {
        StyledValue { style: Cow::Owned(self), value }
    }
}

/// A value that can be printed using the given styles.
pub struct StyledValue<'a, T> {
    style: Cow<'a, Style>,
    value: T,
}

/// 在缓存中设置指定颜色,然后执行 f()函数,执行成功后清空缓存
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

        // f() 执行成功后, 再执行 reset()操作
        write.and(reset)
    }
}

macro_rules! impl_styled_value_fmt {
    // path用来匹配一个路径。
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

/// 下面的fmt操作只对 self.value 生效
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

/// 展开后格式如下
/// impl<'a, T: fmt::Debug> fmt::Debug for StyledValue<'a, T> {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         self.write_fmt(|| T::fmt(&self.value, f))
///     }
/// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fmt::BufferWriter;

    #[test]
    fn test_style() {
        let buffer_writer = BufferWriter::stderr(false, WriteStyle::Auto);

        let buffer = buffer_writer.buffer();

        let style = Style {
            buf: Rc::new(RefCell::new(buffer)),
            spec: ColorSpec::new(),
        };
        println!("{:?}", style);

        let cow: Cow<'_, Style> = Cow::Owned(style);

        let _x = cow.into_owned();
    }
}

use super::Buffer;
use super::Color;
use crate::fmt::writer::WriteStyle;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use termcolor::{self, ColorChoice, ColorSpec};

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
    ///
    /// The same `Style` can be used to print multiple different values.
    pub fn value<T>(&self, value: T) -> StyledValue<T> {
        StyledValue { style: Cow::Borrowed(self), value }
    }

    /// Wrap a value in the style by taking ownership of it.
    pub(crate) fn into_value<T>(self, value: T) -> StyledValue<'static, T> {
        StyledValue { style: Cow::Owned(self), value }
    }
}

/// A value that can be printed using the given styles.
pub struct StyledValue<'a, T> {
    style: Cow<'a, Style>,
    value: T,
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

impl WriteStyle {
    fn into_color_choice(self) -> ColorChoice {
        match self {
            WriteStyle::Always => ColorChoice::Always,
            WriteStyle::Auto => ColorChoice::Auto,
            WriteStyle::Never => ColorChoice::Never,
        }
    }
}

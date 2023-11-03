use crate::fmt::WriteStyle;
use termcolor::{ColorSpec, ColorChoice};
use std::rc::Rc;
use std::cell::RefCell;
use super::Buffer;

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
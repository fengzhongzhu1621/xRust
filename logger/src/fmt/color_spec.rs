use super::Color;

/// A color specification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorSpec {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    bold: bool,
    intense: bool,
    underline: bool,
    dimmed: bool,
    italic: bool,
    reset: bool,
}

impl Default for ColorSpec {
    fn default() -> ColorSpec {
        ColorSpec {
            fg_color: None,
            bg_color: None,
            bold: false,
            intense: false,
            underline: false,
            dimmed: false,
            italic: false,
            reset: true,
        }
    }
}

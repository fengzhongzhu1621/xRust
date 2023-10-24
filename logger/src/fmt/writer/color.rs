use termcolor;
// The `Color` type is copied from https://github.com/BurntSushi/termcolor

/// The set of available colors for the terminal foreground/background.
///
/// The `Ansi256` and `Rgb` colors will only output the correct codes when
/// paired with the `Ansi` `WriteColor` implementation.
///
/// The `Ansi256` and `Rgb` color types are not supported when writing colors
/// on Windows using the console. If they are used on Windows, then they are
/// silently ignored and no colors will be emitted.
///
/// This set may expand over time.
///
/// This type has a `FromStr` impl that can parse colors from their human
/// readable form. The format is as follows:
///
/// 1. Any of the explicitly listed colors in English. They are matched
///    case insensitively.
/// 2. A single 8-bit integer, in either decimal or hexadecimal format.
/// 3. A triple of 8-bit integers separated by a comma, where each integer is
///    in decimal or hexadecimal format.
///
/// Hexadecimal numbers are written with a `0x` prefix.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn into_termcolor(self) -> termcolor::Color {
        match self {
            Color::Black => termcolor::Color::Black,
            Color::Blue => termcolor::Color::Blue,
            Color::Green => termcolor::Color::Green,
            Color::Red => termcolor::Color::Red,
            Color::Cyan => termcolor::Color::Cyan,
            Color::Magenta => termcolor::Color::Magenta,
            Color::Yellow => termcolor::Color::Yellow,
            Color::White => termcolor::Color::White,
            Color::Ansi256(value) => termcolor::Color::Ansi256(value),
            Color::Rgb(r, g, b) => termcolor::Color::Rgb(r, g, b),
        }
    }
}

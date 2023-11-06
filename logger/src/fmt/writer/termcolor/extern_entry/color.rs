use termcolor;

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

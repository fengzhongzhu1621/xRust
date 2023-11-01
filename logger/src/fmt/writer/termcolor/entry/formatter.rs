use termcolor::{self, ColorSpec, WriteColor};

impl Formatter {
    /// Begin a new [`Style`].
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

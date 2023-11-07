use super::{is_stderr, is_stdout, BufferWriter, Writer};
use super::{parse_write_style, Target, WritableTarget, WriteStyle};
use std::mem;

/// A builder for a terminal writer.
///
/// The target and style choice can be configured before building.
#[derive(Debug)]
pub struct Builder {
    target: WritableTarget,
    write_style: WriteStyle,
    is_test: bool,
    built: bool,
}

impl Builder {
    /// Initialize the writer builder with defaults.
    pub fn new() -> Self {
        Builder {
            target: Default::default(),
            write_style: Default::default(),
            is_test: false,
            built: false,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

impl Builder {
    /// Set the target to write to.
    pub fn target(&mut self, target: Target) -> &mut Self {
        // Target -> WritableTarget
        self.target = target.into();
        self
    }

    /// Parses a style choice string.
    /// 将字符串转换为 WriteStyle
    pub fn parse_write_style(&mut self, write_style: &str) -> &mut Self {
        self.write_style(parse_write_style(write_style))
    }

    /// Whether or not to print style characters when writing.
    pub fn write_style(&mut self, write_style: WriteStyle) -> &mut Self {
        self.write_style = write_style;
        self
    }

    /// Build a terminal writer.
    pub fn build(&mut self) -> Writer {
        assert!(!self.built, "attempt to re-use consumed builder");
        self.built = true;

        let color_choice = match self.write_style {
            WriteStyle::Auto => {
                if match &self.target {
                    WritableTarget::Stderr => is_stderr(),
                    WritableTarget::Stdout => is_stdout(),
                    WritableTarget::Pipe(_) => false,
                } {
                    WriteStyle::Auto
                } else {
                    WriteStyle::Never
                }
            }
            color_choice => color_choice,
        };

        let writer = match mem::take(&mut self.target) {
            WritableTarget::Stderr => {
                BufferWriter::stderr(self.is_test, color_choice)
            }
            WritableTarget::Stdout => {
                BufferWriter::stdout(self.is_test, color_choice)
            }
            WritableTarget::Pipe(pipe) => {
                BufferWriter::pipe(color_choice, pipe)
            }
        };

        Writer { inner: writer, write_style: self.write_style }
    }
}

impl Builder {
    /// Whether or not to capture logs for `cargo test`.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_test(&mut self, is_test: bool) -> &mut Self {
        self.is_test = is_test;
        self
    }
}

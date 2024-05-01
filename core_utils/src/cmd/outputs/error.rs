use super::output::{Output, OutputCause};
use crate::cmd::color::Palette;
use crate::str::DebugBytes;
use bstr;
use std::error::Error;
use std::{fmt, process};

#[derive(Debug)]
pub struct OutputError {
    cmd: Option<String>,          // 执行的命令
    stdin: Option<bstr::BString>, // 输入参数
    cause: OutputCause,           // 输出类型
}

impl OutputError {
    /// Convert [`Output`] into an [`Error`].
    ///
    /// [`Output`]: std::process::Output
    /// [`Error`]: std::error::Error
    pub fn new(output: process::Output) -> Self {
        Self {
            cmd: None,
            stdin: None,
            cause: OutputCause::Expected(Output { output }),
        }
    }

    /// For errors that happen in creating a [`Output`].
    ///
    /// [`Output`]: std::process::Output
    pub fn with_cause<E>(cause: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            cmd: None,
            stdin: None,
            cause: OutputCause::Unexpected(Box::new(cause)),
        }
    }

    /// Add the command line for additional context.
    pub fn set_cmd(mut self, cmd: String) -> Self {
        self.cmd = Some(cmd);
        self
    }

    /// Add the `stdin` for additional context.
    pub fn set_stdin(mut self, stdin: Vec<u8>) -> Self {
        self.stdin = Some(bstr::BString::from(stdin));
        self
    }

    /// Access the contained [`Output`].
    pub fn as_output(&self) -> Option<&process::Output> {
        match self.cause {
            OutputCause::Expected(ref e) => Some(&e.output),
            OutputCause::Unexpected(_) => None,
        }
    }
}

impl Error for OutputError {}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::color();
        // 打印执行的命令
        if let Some(ref cmd) = self.cmd {
            writeln!(
                f,
                "{:#}={:#}",
                palette.key("command"),
                palette.value(cmd)
            )?;
        }
        // 打印输入参数
        if let Some(ref stdin) = self.stdin {
            writeln!(
                f,
                "{:#}={:#}",
                palette.key("stdin"),
                palette.value(DebugBytes::new(stdin))
            )?;
        }
        // 打印命令输出
        write!(f, "{:#}", self.cause)
    }
}

use bstr::ByteSlice;
use predicates::prelude::*;
use std::error::Error;
use std::fmt;
use std::process;
pub type OutputResult = Result<process::Output, OutputError>;

#[derive(Debug)]
pub struct OutputError {
    cmd: Option<String>,
    stdin: Option<bstr::BString>,
    cause: OutputCause,
}

#[derive(Debug)]
enum OutputCause {
    Expected(Output),
    Unexpected(Box<dyn Error + Send + Sync + 'static>),
}

impl fmt::Display for OutputCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OutputCause::Expected(ref e) => write!(f, "{:#}", e),
            OutputCause::Unexpected(ref e) => write!(f, "{:#}", e),
        }
    }
}

#[derive(Debug)]
struct Output {
    output: process::Output,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        output_fmt(&self.output, f)
    }
}

pub(crate) fn output_fmt(
    output: &process::Output,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let palette = crate::Palette::color();
    if let Some(code) = output.status.code() {
        writeln!(f, "{:#}={:#}", palette.key("code"), palette.value(code))?;
    } else {
        writeln!(
            f,
            "{:#}={:#}",
            palette.key("code"),
            palette.value("<interrupted>")
        )?;
    }

    write!(
        f,
        "{:#}={:#}\n{:#}={:#}\n",
        palette.key("stdout"),
        palette.value(DebugBytes::new(&output.stdout)),
        palette.key("stderr"),
        palette.value(DebugBytes::new(&output.stderr)),
    )?;
    Ok(())
}

/// Converts a type to an [`OutputResult`].
pub trait OutputOkExt
where
    Self: ::std::marker::Sized,
{
    fn ok(self) -> OutputResult;
}

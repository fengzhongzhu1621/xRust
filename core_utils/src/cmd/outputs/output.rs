use super::color::Palette;
use crate::str::DebugBytes;
use std::error::Error;
use std::{fmt, process};

#[derive(Debug)]
pub enum OutputCause {
    Expected(Output), // 输出符合要求
    Unexpected(Box<dyn Error + Send + Sync + 'static>), // 输出不符合要求
}

impl fmt::Display for OutputCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OutputCause::Expected(ref e) => write!(f, "{:#}", e),
            OutputCause::Unexpected(ref e) => write!(f, "{:#}", e),
        }
    }
}

/// 进程输出
#[derive(Debug)]
pub struct Output {
    pub(crate) output: process::Output,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        output_fmt(&self.output, f)
    }
}

/// 打印进程输出的内容
pub(crate) fn output_fmt(
    output: &process::Output,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    // 创建调色板
    let palette = Palette::color();

    if let Some(code) = output.status.code() {
        // 打印进程正常终止的错误码
        writeln!(f, "{:#}={:#}", palette.key("code"), palette.value(code))?;
    } else {
        // 如果是通过信号终止，此时没有错误码，则打印 <interrupted>
        writeln!(
            f,
            "{:#}={:#}",
            palette.key("code"),
            palette.value("<interrupted>")
        )?;
    }

    // 打印标准输出和错误输出
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

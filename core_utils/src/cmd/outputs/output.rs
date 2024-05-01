use super::error::OutputError;
use crate::cmd::color::Palette;
use crate::str::DebugBytes;
use std::error::Error;
use std::{fmt, process};

pub type OutputResult = Result<process::Output, OutputError>;

pub trait OutputOkExt
where
    Self: ::std::marker::Sized,
{
    fn ok(self) -> OutputResult;

    fn unwrap(self) -> process::Output {
        match self.ok() {
            Ok(output) => output,
            Err(err) => panic!("{}", err),
        }
    }

    fn unwrap_err(self) -> OutputError {
        match self.ok() {
            Ok(output) => panic!(
                "Command completed successfully\nstdout=```{}```",
                DebugBytes::new(&output.stdout)
            ),
            Err(err) => err,
        }
    }
}

impl OutputOkExt for process::Output {
    /// 用于自定义 ok 的错误
    fn ok(self) -> OutputResult {
        if self.status.success() {
            Ok(self)
        } else {
            let error = OutputError::new(self);
            Err(error)
        }
    }
}

impl<'c> OutputOkExt for &'c mut process::Command {
    fn ok(self) -> OutputResult {
        // 将 output() 方法的错误转换未 OutputError类型
        let output = self.output().map_err(OutputError::with_cause)?;
        if output.status.success() {
            Ok(output)
        } else {
            // 错误记录命令
            let error =
                OutputError::new(output).set_cmd(format!("{:?}", self));
            Err(error)
        }
    }

    fn unwrap_err(self) -> OutputError {
        match self.ok() {
            Ok(output) => panic!(
                "Completed successfully:\ncommand=`{:?}`\nstdout=```{}```",
                self,
                DebugBytes::new(&output.stdout)
            ),
            Err(err) => err,
        }
    }
}

#[derive(Debug)]
pub enum OutputCause {
    Expected(Output), // 命令执行失败
    Unexpected(Box<dyn Error + Send + Sync + 'static>), // 执行 process::Command::output() 函数执行错误
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let result = process::Command::new("echo").args(&["42"]).ok();
        assert!(result.is_ok());
    }
}

use super::assert::Assert;
use super::reason::AssertReason;
use crate::str::DebugBytes;
use std::error::Error;
use std::fmt;

/// [`Assert`] error (see [`AssertResult`]).
#[derive(Debug)]
pub struct AssertError {
    pub(crate) assert: Assert,
    pub(crate) reason: AssertReason,
}

impl AssertError {
    // track_caller 可以应用于除程序入口函数 fn main 之外的任何带有 "Rust" ABI 的函数。
    // 使用这个属性标记了的函数中发生panic之后，异常堆栈中可以很详细的得知panic的地点（比如文件、行数、列数）。
    // 以前的版本调用Option和Result的unwrap()函数如果发生panic，错误堆栈里不会显示具体panic的地点，
    // 直到1.42版Option和Result的unwrap()函数使用#[track_caller]标记之后解决了这个问题。
    // 现在#[track_caller]稳定之后，开发者可以给任何可能发生panic的函数采用#[track_caller]标记了。
    #[track_caller]
    pub(crate) fn panic<T>(self) -> T {
        panic!("{}", self)
    }

    pub fn assert(self) -> Assert {
        self.assert
    }
}

impl Error for AssertError {}

impl fmt::Display for AssertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.reason {
            AssertReason::UnexpectedFailure { actual_code } => writeln!(
                f,
                "Unexpected failure.\ncode={}\nstderr=```{}```",
                actual_code
                    .map(|actual_code| actual_code.to_string())
                    .unwrap_or_else(|| "<interrupted>".to_owned()),
                DebugBytes::new(&self.assert.output.stderr),
            ),
            AssertReason::UnexpectedSuccess => {
                writeln!(f, "Unexpected success")
            }
            AssertReason::UnexpectedCompletion => {
                writeln!(f, "Unexpected completion")
            }
            AssertReason::CommandInterrupted => {
                writeln!(f, "Command interrupted")
            }
            AssertReason::UnexpectedReturnCode { case_tree } => {
                writeln!(f, "Unexpected return code, failed {}", case_tree)
            }
            AssertReason::UnexpectedStdout { case_tree } => {
                writeln!(f, "Unexpected stdout, failed {}", case_tree)
            }
            AssertReason::UnexpectedStderr { case_tree } => {
                writeln!(f, "Unexpected stderr, failed {}", case_tree)
            }
        }?;
        write!(f, "{}", self.assert)
    }
}

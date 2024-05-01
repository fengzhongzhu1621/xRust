use super::assert::Assert;
use std::process;

pub trait OutputAssertExt {
    /// Wrap with an interface for that provides assertions on the [`Output`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::cargo_bin("bin_fixture")
    ///     .unwrap();
    /// cmd.assert()
    ///     .success();
    /// ```
    ///
    /// [`Output`]: std::process::Output
    fn assert(self) -> Assert;
}

impl OutputAssertExt for process::Output {
    fn assert(self) -> Assert {
        Assert::new(self)
    }
}

impl<'c> OutputAssertExt for &'c mut process::Command {
    fn assert(self) -> Assert {
        let output = match self.output() {
            Ok(output) => output,
            Err(err) => {
                panic!("Failed to spawn {:?}: {}", self, err);
            }
        };
        Assert::new(output).append_context("command", format!("{:?}", self))
    }
}

use crate::platform::sys;
use std::io;
use std::process::{Child, ExitStatus};
use std::time::Duration;

/// Extension methods for the standard `std::process::Child` type.
pub trait ChildExt {
    /// Deprecated, use `wait_timeout` instead.
    #[doc(hidden)]
    fn wait_timeout_ms(&mut self, ms: u32) -> io::Result<Option<ExitStatus>> {
        self.wait_timeout(Duration::from_millis(ms as u64))
    }

    /// Wait for this child to exit, timing out after the duration `dur` has
    /// elapsed.
    ///
    /// If `Ok(None)` is returned then the timeout period elapsed without the
    /// child exiting, and if `Ok(Some(..))` is returned then the child exited
    /// with the specified exit code.
    fn wait_timeout(
        &mut self,
        dur: Duration,
    ) -> io::Result<Option<ExitStatus>>;
}

impl ChildExt for Child {
    fn wait_timeout(
        &mut self,
        dur: Duration,
    ) -> io::Result<Option<ExitStatus>> {
        drop(self.stdin.take());
        sys::wait_timeout(self, dur)
    }
}

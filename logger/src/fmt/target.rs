use std::sync::Mutex;
use std::{fmt, io};

/// Log target, either `stdout`, `stderr` or a custom pipe.
#[non_exhaustive]
pub enum Target {
    /// Logs will be sent to standard output.
    Stdout,
    /// Logs will be sent to standard error.
    Stderr,
    /// Logs will be sent to a custom pipe.
    Pipe(Box<dyn io::Write + Send + 'static>),
}

impl Default for Target {
    fn default() -> Self {
        Target::Stderr
    }
}

impl fmt::Debug for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Stdout => "stdout",
                Self::Stderr => "stderr",
                Self::Pipe(_) => "pipe",
            }
        )
    }
}

/// Log target, either `stdout`, `stderr` or a custom pipe.
///
/// Same as `Target`, except the pipe is wrapped in a mutex for interior mutability.
pub enum WritableTarget {
    /// Logs will be sent to standard output.
    Stdout,
    /// Logs will be sent to standard error.
    Stderr,
    /// Logs will be sent to a custom pipe.
    Pipe(Box<Mutex<dyn io::Write + Send + 'static>>),
}

/// WritableTarget 构造函数
impl Default for WritableTarget {
    fn default() -> Self {
        Self::from(Target::default())
    }
}

/// 实现了 WritableTarget::from(Target) 方法
impl From<Target> for WritableTarget {
    fn from(target: Target) -> Self {
        match target {
            Target::Stdout => Self::Stdout,
            Target::Stderr => Self::Stderr,
            Target::Pipe(pipe) => Self::Pipe(Box::new(Mutex::new(pipe))),
        }
    }
}

impl fmt::Debug for WritableTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Stdout => "stdout",
                Self::Stderr => "stderr",
                Self::Pipe(_) => "pipe",
            }
        )
    }
}

use std::error::Error;
use std::fmt;
use std::path;

/// Error when finding crate binary.
#[derive(Debug)]
pub struct CargoError {
    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl CargoError {
    /// Wrap the underlying error for passing up.
    /// 包装了其他的错误
    pub fn with_cause<E>(cause: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let cause = Box::new(cause);
        Self { cause: Some(cause) }
    }
}

impl Error for CargoError {}

impl fmt::Display for CargoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cause) = self.cause {
            writeln!(f, "Cause: {}", cause)?;
        }
        Ok(())
    }
}

/// Error when finding crate binary.
#[derive(Debug)]
pub(crate) struct NotFoundError {
    pub(crate) path: path::PathBuf,
}

impl Error for NotFoundError {}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cargo command not found: {}", self.path.display())
    }
}

use crate::platform;
use std::{error, fmt, io};

/// Ctrl-C error.
#[derive(Debug)]
pub enum CtrlcError {
    /// Signal could not be found from the system.
    NoSuchSignal(crate::signal::SignalType),
    /// Ctrl-C signal handler already registered.
    MultipleHandlers,
    /// Unexpected system error.
    System(io::Error),
}

impl CtrlcError {
    fn describe(&self) -> &str {
        match *self {
            CtrlcError::NoSuchSignal(_) => {
                "Signal could not be found from the system"
            }
            CtrlcError::MultipleHandlers => {
                "Ctrl-C signal handler already registered"
            }
            CtrlcError::System(_) => "Unexpected system error",
        }
    }
}

impl fmt::Display for CtrlcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ctrl-C error: {}", self.describe())
    }
}

impl error::Error for CtrlcError {
    fn description(&self) -> &str {
        self.describe()
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            CtrlcError::System(ref e) => Some(e),
            _ => None,
        }
    }
}

// platform::Error -> CtrlcError
impl From<platform::sys::IOError> for CtrlcError {
    fn from(e: platform::sys::IOError) -> CtrlcError {
        #[cfg(not(windows))]
        if e == platform::sys::IOError::EEXIST {
            return CtrlcError::MultipleHandlers;
        }

        let system_error = io::Error::new(io::ErrorKind::Other, e);
        CtrlcError::System(system_error)
    }
}

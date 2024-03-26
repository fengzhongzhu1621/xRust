use super::{Error, OsString};

/// Conversion of anything into an owned OS-native string. If allocation fails,
/// an error shall be returned.
pub trait IntoOsString {
    /// Converts with possible allocation error.
    fn into_os_string(self) -> Result<OsString, Error>;
}

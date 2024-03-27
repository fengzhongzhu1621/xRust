use super::sys::{Error, OsStr, OsString};
use core::{fmt, ops::Deref};

/// Conversion of anything into an owned OS-native string. If allocation fails,
/// an error shall be returned.
pub trait IntoOsString {
    /// Converts with possible allocation error.
    fn into_os_string(self) -> Result<OsString, Error>;
}

/// Conversion of anything to an either borrowed or owned OS-native string. If
/// allocation fails, an error shall be returned.
pub trait ToOsStr {
    /// Converts with possible allocation error.
    fn to_os_str(&self) -> Result<EitherOsStr, Error>;
}

impl IntoOsString for OsString {
    fn into_os_string(self) -> Result<OsString, Error> {
        Ok(self)
    }
}

impl fmt::Debug for OsString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self.as_ref())
    }
}

impl fmt::Display for OsString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.as_ref())
    }
}

/// 定义了一个通用的操作系统本地字符串
/// Either borrowed or owned allocation of an OS-native string.
#[derive(Debug)]
pub enum EitherOsStr<'str> {
    /// Borrowed allocation.
    Borrowed(&'str OsStr),
    /// Owned allocation.
    Owned(OsString),
}

//// 获得 EitherOsStr 的引用
impl<'str> AsRef<OsStr> for EitherOsStr<'str> {
    fn as_ref(&self) -> &OsStr {
        match self {
            Self::Borrowed(str) => str, // &OsStr -> &OsSTr
            Self::Owned(string) => string.as_ref(), // OsString -> &OsStr
        }
    }
}

/// EitherOsStr 解引用
impl<'str> Deref for EitherOsStr<'str> {
    type Target = OsStr;

    // &OsStr -> &OsStr -> OsStr
    // OsString -> &OsStr -> OsStr
    fn deref(&self) -> &OsStr {
        self.as_ref()
    }
}

// EitherOsStr -> Result<EitherOsStr, Error>
impl<'str> ToOsStr for EitherOsStr<'str> {
    fn to_os_str(&self) -> Result<EitherOsStr, Error> {
        Ok(match self {
            EitherOsStr::Borrowed(str) => EitherOsStr::Borrowed(str),
            // OsString -> EitherOsStr::Borrowed -> Result<OsString, Error>
            EitherOsStr::Owned(string) => {
                EitherOsStr::Owned(string.to_os_str()?.into_os_string()?)
            }
        })
    }
}

/// OsString -> EitherOsStr::Borrowed
impl ToOsStr for OsString {
    fn to_os_str(&self) -> Result<EitherOsStr, Error> {
        Ok(EitherOsStr::Borrowed(self.as_ref()))
    }
}

/// EitherOsStr -> Result<OsString, Error>
impl<'str> IntoOsString for EitherOsStr<'str> {
    fn into_os_string(self) -> Result<OsString, Error> {
        match self {
            Self::Borrowed(str) => str.into_os_string(), // &OsStr -> Result<OsString, Error>
            Self::Owned(string) => Ok(string), // OsString -> Ok(OsString)
        }
    }
}

// OsString 复制
impl Clone for OsString {
    fn clone(&self) -> Self {
        self.to_os_str()
            .and_then(|str| str.into_os_string())
            .expect("Allocation error")
    }
}

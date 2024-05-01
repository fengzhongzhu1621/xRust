use std;

/// An error that occurs when UTF-8 decoding fails.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Utf8Error {
    pub(crate) valid_up_to: usize,
    pub(crate) error_len: Option<usize>,
}

impl Utf8Error {
    /// Returns the byte index of the position immediately following the last
    /// valid UTF-8 byte.
    #[inline]
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }

    /// Returns the total number of invalid UTF-8 bytes immediately following
    /// the position returned by `valid_up_to`. This value is always at least
    /// `1`, but can be up to `3` if bytes form a valid prefix of some UTF-8
    /// encoded codepoint.
    ///
    /// If the end of the original input was found before a valid UTF-8 encoded
    /// codepoint could be completed, then this returns `None`. This is useful
    /// when processing streams, where a `None` value signals that more input
    /// might be needed.
    #[inline]
    pub fn error_len(&self) -> Option<usize> {
        self.error_len
    }
}

impl std::error::Error for Utf8Error {
    fn description(&self) -> &str {
        "invalid UTF-8"
    }
}

impl std::fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid UTF-8 found at byte offset {}", self.valid_up_to)
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct FromUtf8Error {
    pub(crate) original: Vec<u8>,
    pub(crate) err: Utf8Error,
}

impl FromUtf8Error {
    /// Return the original bytes as a slice that failed to convert to a
    /// `String`.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.original
    }

    /// Consume this error and return the original byte string that failed to
    /// convert to a `String`.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.original
    }

    /// Return the underlying UTF-8 error that occurred. This error provides
    /// information on the nature and location of the invalid UTF-8 detected.
    #[inline]
    pub fn utf8_error(&self) -> &Utf8Error {
        &self.err
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromUtf8Error {
    #[inline]
    fn description(&self) -> &str {
        "invalid UTF-8 vector"
    }
}

impl std::fmt::Display for FromUtf8Error {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}
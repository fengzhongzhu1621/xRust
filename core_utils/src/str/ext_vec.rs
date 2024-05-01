use alloc::{borrow::Cow, string::String, vec, vec::Vec};
use core::{iter, ops, ptr};
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use super::{
    error::FromUtf8Error, escape_bytes::UnescapeBytes, ext_slice::ByteSlice,
    utf8,
};

/// Concatenate the elements given by the iterator together into a single
/// `Vec<u8>`.
#[inline]
pub fn concat<T, I>(elements: I) -> Vec<u8>
where
    T: AsRef<[u8]>,
    I: IntoIterator<Item = T>,
{
    let mut dest = vec![];
    for element in elements {
        dest.push_str(element);
    }
    dest
}

/// Join the elements given by the iterator with the given separator into a
/// single `Vec<u8>`.
#[inline]
pub fn join<B, T, I>(separator: B, elements: I) -> Vec<u8>
where
    B: AsRef<[u8]>,
    T: AsRef<[u8]>,
    I: IntoIterator<Item = T>,
{
    let mut it = elements.into_iter();
    let mut dest = vec![];
    match it.next() {
        None => return dest,
        Some(first) => {
            dest.push_str(first);
        }
    }
    for element in it {
        dest.push_str(&separator);
        dest.push_str(element);
    }
    dest
}

impl ByteVec for Vec<u8> {
    #[inline]
    fn as_vec(&self) -> &Vec<u8> {
        self
    }

    #[inline]
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        self
    }
}

/// Ensure that callers cannot implement `ByteSlice` by making an
/// umplementable trait its super trait.
mod private {
    pub trait Sealed {}
}
impl private::Sealed for Vec<u8> {}

/// A trait that extends `Vec<u8>` with string oriented methods.
pub trait ByteVec: private::Sealed {
    /// A method for accessing the raw vector bytes of this type. This is
    /// always a no-op and callers shouldn't care about it. This only exists
    /// for making the extension trait work.
    #[doc(hidden)]
    fn as_vec(&self) -> &Vec<u8>;

    /// A method for accessing the raw vector bytes of this type, mutably. This
    /// is always a no-op and callers shouldn't care about it. This only exists
    /// for making the extension trait work.
    #[doc(hidden)]
    fn as_vec_mut(&mut self) -> &mut Vec<u8>;

    /// A method for consuming ownership of this vector. This is always a no-op
    /// and callers shouldn't care about it. This only exists for making the
    /// extension trait work.
    #[doc(hidden)]
    fn into_vec(self) -> Vec<u8>
    where
        Self: Sized;

    /// Create a new owned byte string from the given byte slice.
    #[inline]
    fn from_slice<B: AsRef<[u8]>>(bytes: B) -> Vec<u8> {
        bytes.as_ref().to_vec()
    }

    /// Create a new byte string from an owned OS string.
    #[inline]
    #[cfg(feature = "std")]
    fn from_os_string(os_str: OsString) -> Result<Vec<u8>, OsString> {
        #[cfg(unix)]
        #[inline]
        fn imp(os_str: OsString) -> Result<Vec<u8>, OsString> {
            use std::os::unix::ffi::OsStringExt;

            Ok(Vec::from(os_str.into_vec()))
        }

        #[cfg(not(unix))]
        #[inline]
        fn imp(os_str: OsString) -> Result<Vec<u8>, OsString> {
            os_str.into_string().map(Vec::from)
        }

        imp(os_str)
    }

    /// Lossily create a new byte string from an OS string slice.
    #[inline]
    #[cfg(feature = "std")]
    fn from_os_str_lossy<'a>(os_str: &'a OsStr) -> Cow<'a, [u8]> {
        #[cfg(unix)]
        #[inline]
        fn imp<'a>(os_str: &'a OsStr) -> Cow<'a, [u8]> {
            use std::os::unix::ffi::OsStrExt;

            Cow::Borrowed(os_str.as_bytes())
        }

        #[cfg(not(unix))]
        #[inline]
        fn imp<'a>(os_str: &'a OsStr) -> Cow<'a, [u8]> {
            match os_str.to_string_lossy() {
                Cow::Borrowed(x) => Cow::Borrowed(x.as_bytes()),
                Cow::Owned(x) => Cow::Owned(Vec::from(x)),
            }
        }

        imp(os_str)
    }

    /// Create a new byte string from an owned file path.
    #[inline]
    #[cfg(feature = "std")]
    fn from_path_buf(path: PathBuf) -> Result<Vec<u8>, PathBuf> {
        Vec::from_os_string(path.into_os_string()).map_err(PathBuf::from)
    }

    /// Lossily create a new byte string from a file path.
    #[inline]
    #[cfg(feature = "std")]
    fn from_path_lossy<'a>(path: &'a Path) -> Cow<'a, [u8]> {
        Vec::from_os_str_lossy(path.as_os_str())
    }

    /// Unescapes the given string into its raw bytes.
    #[inline]
    fn unescape_bytes<S: AsRef<str>>(escaped: S) -> Vec<u8> {
        let s = escaped.as_ref();
        UnescapeBytes::new(s.chars()).collect()
    }

    /// Appends the given byte to the end of this byte string.
    #[inline]
    fn push_byte(&mut self, byte: u8) {
        self.as_vec_mut().push(byte);
    }

    /// Appends the given `char` to the end of this byte string.
    #[inline]
    fn push_char(&mut self, ch: char) {
        if ch.len_utf8() == 1 {
            self.push_byte(ch as u8);
            return;
        }
        self.as_vec_mut()
            .extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes());
    }

    /// Appends the given slice to the end of this byte string. This accepts
    /// any type that be converted to a `&[u8]`. This includes, but is not
    /// limited to, `&str`, `&BStr`, and of course, `&[u8]` itself.
    #[inline]
    fn push_str<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.as_vec_mut().extend_from_slice(bytes.as_ref());
    }

    /// Converts a `Vec<u8>` into a `String` if and only if this byte string is
    /// valid UTF-8.
    #[inline]
    fn into_string(self) -> Result<String, FromUtf8Error>
    where
        Self: Sized,
    {
        match utf8::validate(self.as_vec()) {
            Err(err) => Err(FromUtf8Error { original: self.into_vec(), err }),
            Ok(()) => {
                // SAFETY: This is safe because of the guarantees provided by
                // utf8::validate.
                unsafe { Ok(self.into_string_unchecked()) }
            }
        }
    }

    /// Lossily converts a `Vec<u8>` into a `String`. If this byte string
    /// contains invalid UTF-8, then the invalid bytes are replaced with the
    /// Unicode replacement codepoint.
    #[inline]
    fn into_string_lossy(self) -> String
    where
        Self: Sized,
    {
        match self.as_vec().to_str_lossy() {
            Cow::Borrowed(_) => {
                // SAFETY: to_str_lossy() returning a Cow::Borrowed guarantees
                // the entire string is valid utf8.
                unsafe { self.into_string_unchecked() }
            }
            Cow::Owned(s) => s,
        }
    }

    /// Unsafely convert this byte string into a `String`, without checking for
    /// valid UTF-8.
    #[inline]
    unsafe fn into_string_unchecked(self) -> String
    where
        Self: Sized,
    {
        String::from_utf8_unchecked(self.into_vec())
    }

    /// Converts this byte string into an OS string, in place.
    #[cfg(feature = "std")]
    #[inline]
    fn into_os_string(self) -> Result<OsString, FromUtf8Error>
    where
        Self: Sized,
    {
        #[cfg(unix)]
        #[inline]
        fn imp(v: Vec<u8>) -> Result<OsString, FromUtf8Error> {
            use std::os::unix::ffi::OsStringExt;

            Ok(OsString::from_vec(v))
        }

        #[cfg(not(unix))]
        #[inline]
        fn imp(v: Vec<u8>) -> Result<OsString, FromUtf8Error> {
            v.into_string().map(OsString::from)
        }

        imp(self.into_vec())
    }

    /// Lossily converts this byte string into an OS string, in place.
    #[inline]
    #[cfg(feature = "std")]
    fn into_os_string_lossy(self) -> OsString
    where
        Self: Sized,
    {
        #[cfg(unix)]
        #[inline]
        fn imp(v: Vec<u8>) -> OsString {
            use std::os::unix::ffi::OsStringExt;

            OsString::from_vec(v)
        }

        #[cfg(not(unix))]
        #[inline]
        fn imp(v: Vec<u8>) -> OsString {
            OsString::from(v.into_string_lossy())
        }

        imp(self.into_vec())
    }

    /// Converts this byte string into an owned file path, in place.
    #[cfg(feature = "std")]
    #[inline]
    fn into_path_buf(self) -> Result<PathBuf, FromUtf8Error>
    where
        Self: Sized,
    {
        self.into_os_string().map(PathBuf::from)
    }

    /// Lossily converts this byte string into an owned file path, in place.
    #[inline]
    #[cfg(feature = "std")]
    fn into_path_buf_lossy(self) -> PathBuf
    where
        Self: Sized,
    {
        PathBuf::from(self.into_os_string_lossy())
    }

    /// Removes the last byte from this `Vec<u8>` and returns it.
    #[inline]
    fn pop_byte(&mut self) -> Option<u8> {
        self.as_vec_mut().pop()
    }

    /// Removes the last codepoint from this `Vec<u8>` and returns it.
    #[inline]
    fn pop_char(&mut self) -> Option<char> {
        let (ch, size) = utf8::decode_last_lossy(self.as_vec());
        if size == 0 {
            return None;
        }
        let new_len = self.as_vec().len() - size;
        self.as_vec_mut().truncate(new_len);
        Some(ch)
    }

    /// Removes a `char` from this `Vec<u8>` at the given byte position and
    /// returns it.
    #[inline]
    fn remove_char(&mut self, at: usize) -> char {
        let (ch, size) = utf8::decode_lossy(&self.as_vec()[at..]);
        assert!(
            size > 0,
            "expected {} to be less than {}",
            at,
            self.as_vec().len(),
        );
        self.as_vec_mut().drain(at..at + size);
        ch
    }

    /// Inserts the given codepoint into this `Vec<u8>` at a particular byte
    /// position.
    #[inline]
    fn insert_char(&mut self, at: usize, ch: char) {
        self.insert_str(at, ch.encode_utf8(&mut [0; 4]).as_bytes());
    }

    /// Inserts the given byte string into this byte string at a particular
    /// byte position.
    #[inline]
    fn insert_str<B: AsRef<[u8]>>(&mut self, at: usize, bytes: B) {
        let bytes = bytes.as_ref();
        let len = self.as_vec().len();
        assert!(at <= len, "expected {} to be <= {}", at, len);

        // SAFETY: We'd like to efficiently splice in the given bytes into
        // this byte string. Since we are only working with `u8` elements here,
        // we only need to consider whether our bounds are correct and whether
        // our byte string has enough space.
        self.as_vec_mut().reserve(bytes.len());
        unsafe {
            // Shift bytes after `at` over by the length of `bytes` to make
            // room for it. This requires referencing two regions of memory
            // that may overlap, so we use ptr::copy.
            ptr::copy(
                self.as_vec().as_ptr().add(at),
                self.as_vec_mut().as_mut_ptr().add(at + bytes.len()),
                len - at,
            );
            // Now copy the bytes given into the room we made above. In this
            // case, we know that the given bytes cannot possibly overlap
            // with this byte string since we have a mutable borrow of the
            // latter. Thus, we can use a nonoverlapping copy.
            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.as_vec_mut().as_mut_ptr().add(at),
                bytes.len(),
            );
            self.as_vec_mut().set_len(len + bytes.len());
        }
    }

    /// Removes the specified range in this byte string and replaces it with
    /// the given bytes. The given bytes do not need to have the same length
    /// as the range provided.
    #[inline]
    fn replace_range<R, B>(&mut self, range: R, replace_with: B)
    where
        R: ops::RangeBounds<usize>,
        B: AsRef<[u8]>,
    {
        self.as_vec_mut().splice(range, replace_with.as_ref().iter().cloned());
    }

    /// Creates a draining iterator that removes the specified range in this
    /// `Vec<u8>` and yields each of the removed bytes.
    #[inline]
    fn drain_bytes<R>(&mut self, range: R) -> DrainBytes<'_>
    where
        R: ops::RangeBounds<usize>,
    {
        DrainBytes { it: self.as_vec_mut().drain(range) }
    }
}

/// A draining byte oriented iterator for `Vec<u8>`.
#[derive(Debug)]
pub struct DrainBytes<'a> {
    it: vec::Drain<'a, u8>,
}

impl<'a> iter::FusedIterator for DrainBytes<'a> {}

impl<'a> Iterator for DrainBytes<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        self.it.next()
    }
}

impl<'a> DoubleEndedIterator for DrainBytes<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<u8> {
        self.it.next_back()
    }
}

impl<'a> ExactSizeIterator for DrainBytes<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.it.len()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use alloc::{vec, vec::Vec};

    use crate::str::ext_vec::ByteVec;

    #[test]
    fn insert() {
        let mut s = vec![];
        s.insert_str(0, "foo");
        assert_eq!(s, "foo".as_bytes());

        let mut s = Vec::from("a");
        s.insert_str(0, "foo");
        assert_eq!(s, "fooa".as_bytes());

        let mut s = Vec::from("a");
        s.insert_str(1, "foo");
        assert_eq!(s, "afoo".as_bytes());

        let mut s = Vec::from("foobar");
        s.insert_str(3, "quux");
        assert_eq!(s, "fooquuxbar".as_bytes());

        let mut s = Vec::from("foobar");
        s.insert_str(3, "x");
        assert_eq!(s, "fooxbar".as_bytes());

        let mut s = Vec::from("foobar");
        s.insert_str(0, "x");
        assert_eq!(s, "xfoobar".as_bytes());

        let mut s = Vec::from("foobar");
        s.insert_str(6, "x");
        assert_eq!(s, "foobarx".as_bytes());

        let mut s = Vec::from("foobar");
        s.insert_str(3, "quuxbazquux");
        assert_eq!(s, "fooquuxbazquuxbar".as_bytes());
    }

    #[test]
    #[should_panic]
    fn insert_fail1() {
        let mut s = vec![];
        s.insert_str(1, "foo");
    }

    #[test]
    #[should_panic]
    fn insert_fail2() {
        let mut s = Vec::from("a");
        s.insert_str(2, "foo");
    }

    #[test]
    #[should_panic]
    fn insert_fail3() {
        let mut s = Vec::from("foobar");
        s.insert_str(7, "foo");
    }
}

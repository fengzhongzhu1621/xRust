use core::{iter, slice, str};
use alloc::vec;
use alloc::{borrow::Cow, string::String, vec::Vec};
use std::{ffi::OsStr, path::Path};

use memchr::{memchr, memmem, memrchr};

use super::escape_bytes::EscapeBytes;
use super::ext_vec::ByteVec;
use super::unicode::{
    whitespace_len_fwd, whitespace_len_rev, GraphemeIndices, Graphemes,
    SentenceIndices, Sentences, WordIndices, Words, WordsWithBreakIndices,
    WordsWithBreaks,
};
use super::{helper::*,
    ascii,
    bstr::BStr,
    byteset,
    utf8::{self, CharIndices, Chars, Utf8Chunks},
    error::Utf8Error
};

impl ByteSlice for [u8] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl<const N: usize> ByteSlice for [u8; N] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self
    }
}

/// Ensure that callers cannot implement `ByteSlice` by making an
/// umplementable trait its super trait.
mod private {
    pub trait Sealed {}
}
impl private::Sealed for [u8] {}
impl<const N: usize> private::Sealed for [u8; N] {}

/// A trait that extends `&[u8]` with string oriented methods.
///
/// This trait is sealed and cannot be implemented outside of `bstr`.
pub trait ByteSlice: private::Sealed {
    /// A method for accessing the raw bytes of this type. This is always a
    /// no-op and callers shouldn't care about it. This only exists for making
    /// the extension trait work.
    #[doc(hidden)]
    fn as_bytes(&self) -> &[u8];

    /// A method for accessing the raw bytes of this type, mutably. This is
    /// always a no-op and callers shouldn't care about it. This only exists
    /// for making the extension trait work.
    #[doc(hidden)]
    fn as_bytes_mut(&mut self) -> &mut [u8];

    /// Return this byte slice as a `&BStr`.
    #[inline]
    fn as_bstr(&self) -> &BStr {
        BStr::new(self.as_bytes())
    }

    /// Return this byte slice as a `&mut BStr`.
    #[inline]
    fn as_bstr_mut(&mut self) -> &mut BStr {
        BStr::new_mut(self.as_bytes_mut())
    }

    /// Create an immutable byte string from an OS string slice.
    #[cfg(feature = "std")]
    #[inline]
    fn from_os_str(os_str: &OsStr) -> Option<&[u8]> {
        #[cfg(unix)]
        #[inline]
        fn imp(os_str: &OsStr) -> Option<&[u8]> {
            use std::os::unix::ffi::OsStrExt;

            Some(os_str.as_bytes())
        }

        #[cfg(not(unix))]
        #[inline]
        fn imp(os_str: &OsStr) -> Option<&[u8]> {
            os_str.to_str().map(|s| s.as_bytes())
        }

        imp(os_str)
    }

    /// Create an immutable byte string from a file path.
    #[cfg(feature = "std")]
    #[inline]
    fn from_path(path: &Path) -> Option<&[u8]> {
        Self::from_os_str(path.as_os_str())
    }

    /// Safely convert this byte string into a `&str` if it's valid UTF-8.
    #[inline]
    fn to_str(&self) -> Result<&str, Utf8Error> {
        utf8::validate(self.as_bytes()).map(|_| {
            // SAFETY: This is safe because of the guarantees provided by
            // utf8::validate.
            unsafe { str::from_utf8_unchecked(self.as_bytes()) }
        })
    }

    /// Unsafely convert this byte string into a `&str`, without checking for
    /// valid UTF-8.
    #[inline]
    unsafe fn to_str_unchecked(&self) -> &str {
        str::from_utf8_unchecked(self.as_bytes())
    }

    /// Convert this byte string to a valid UTF-8 string by replacing invalid
    /// UTF-8 bytes with the Unicode replacement codepoint (`U+FFFD`).
    #[inline]
    fn to_str_lossy(&self) -> Cow<'_, str> {
        match utf8::validate(self.as_bytes()) {
            Ok(()) => {
                // SAFETY: This is safe because of the guarantees provided by
                // utf8::validate.
                unsafe {
                    Cow::Borrowed(str::from_utf8_unchecked(self.as_bytes()))
                }
            }
            Err(err) => {
                let mut lossy = String::with_capacity(self.as_bytes().len());
                let (valid, after) =
                    self.as_bytes().split_at(err.valid_up_to());
                // SAFETY: This is safe because utf8::validate guarantees
                // that all of `valid` is valid UTF-8.
                lossy.push_str(unsafe { str::from_utf8_unchecked(valid) });
                lossy.push_str("\u{FFFD}");
                if let Some(len) = err.error_len() {
                    after[len..].to_str_lossy_into(&mut lossy);
                }
                Cow::Owned(lossy)
            }
        }
    }

    /// Copy the contents of this byte string into the given owned string
    /// buffer, while replacing invalid UTF-8 code unit sequences with the
    /// Unicode replacement codepoint (`U+FFFD`).
    #[inline]
    fn to_str_lossy_into(&self, dest: &mut String) {
        let mut bytes = self.as_bytes();
        dest.reserve(bytes.len());
        loop {
            match utf8::validate(bytes) {
                Ok(()) => {
                    // SAFETY: This is safe because utf8::validate guarantees
                    // that all of `bytes` is valid UTF-8.
                    dest.push_str(unsafe { str::from_utf8_unchecked(bytes) });
                    break;
                }
                Err(err) => {
                    let (valid, after) = bytes.split_at(err.valid_up_to());
                    // SAFETY: This is safe because utf8::validate guarantees
                    // that all of `valid` is valid UTF-8.
                    dest.push_str(unsafe { str::from_utf8_unchecked(valid) });
                    dest.push_str("\u{FFFD}");
                    match err.error_len() {
                        None => break,
                        Some(len) => bytes = &after[len..],
                    }
                }
            }
        }
    }

    /// Create an OS string slice from this byte string.
    #[cfg(feature = "std")]
    #[inline]
    fn to_os_str(&self) -> Result<&OsStr, Utf8Error> {
        #[cfg(unix)]
        #[inline]
        fn imp(bytes: &[u8]) -> Result<&OsStr, Utf8Error> {
            use std::os::unix::ffi::OsStrExt;

            Ok(OsStr::from_bytes(bytes))
        }

        #[cfg(not(unix))]
        #[inline]
        fn imp(bytes: &[u8]) -> Result<&OsStr, Utf8Error> {
            bytes.to_str().map(OsStr::new)
        }

        imp(self.as_bytes())
    }

    /// Lossily create an OS string slice from this byte string.
    #[cfg(feature = "std")]
    #[inline]
    fn to_os_str_lossy(&self) -> Cow<'_, OsStr> {
        #[cfg(unix)]
        #[inline]
        fn imp(bytes: &[u8]) -> Cow<'_, OsStr> {
            use std::os::unix::ffi::OsStrExt;

            Cow::Borrowed(OsStr::from_bytes(bytes))
        }

        #[cfg(not(unix))]
        #[inline]
        fn imp(bytes: &[u8]) -> Cow<OsStr> {
            use std::ffi::OsString;

            match bytes.to_str_lossy() {
                Cow::Borrowed(x) => Cow::Borrowed(OsStr::new(x)),
                Cow::Owned(x) => Cow::Owned(OsString::from(x)),
            }
        }

        imp(self.as_bytes())
    }

    /// Create a path slice from this byte string.
    #[cfg(feature = "std")]
    #[inline]
    fn to_path(&self) -> Result<&Path, Utf8Error> {
        self.to_os_str().map(Path::new)
    }

    /// Lossily create a path slice from this byte string.
    #[cfg(feature = "std")]
    #[inline]
    fn to_path_lossy(&self) -> Cow<'_, Path> {
        use std::path::PathBuf;

        match self.to_os_str_lossy() {
            Cow::Borrowed(x) => Cow::Borrowed(Path::new(x)),
            Cow::Owned(x) => Cow::Owned(PathBuf::from(x)),
        }
    }

    /// Create a new byte string by repeating this byte string `n` times.
    #[inline]
    fn repeatn(&self, n: usize) -> Vec<u8> {
        self.as_bytes().repeat(n)
    }

    /// Returns true if and only if this byte string contains the given needle.
    #[inline]
    fn contains_str<B: AsRef<[u8]>>(&self, needle: B) -> bool {
        self.find(needle).is_some()
    }

    /// Returns true if and only if this byte string has the given prefix.
    #[inline]
    fn starts_with_str<B: AsRef<[u8]>>(&self, prefix: B) -> bool {
        self.as_bytes().starts_with(prefix.as_ref())
    }

    /// Returns true if and only if this byte string has the given suffix.
    #[inline]
    fn ends_with_str<B: AsRef<[u8]>>(&self, suffix: B) -> bool {
        self.as_bytes().ends_with(suffix.as_ref())
    }

    /// Returns the index of the first occurrence of the given needle.
    #[inline]
    fn find<B: AsRef<[u8]>>(&self, needle: B) -> Option<usize> {
        Finder::new(needle.as_ref()).find(self.as_bytes())
    }

    /// Returns the index of the last occurrence of the given needle.
    #[inline]
    fn rfind<B: AsRef<[u8]>>(&self, needle: B) -> Option<usize> {
        FinderReverse::new(needle.as_ref()).rfind(self.as_bytes())
    }

    /// Returns an iterator of the non-overlapping occurrences of the given
    /// needle. The iterator yields byte offset positions indicating the start
    /// of each match.
    #[inline]
    fn find_iter<'h, 'n, B: ?Sized + AsRef<[u8]>>(
        &'h self,
        needle: &'n B,
    ) -> Find<'h, 'n> {
        Find::new(self.as_bytes(), needle.as_ref())
    }

    /// Returns an iterator of the non-overlapping occurrences of the given
    /// needle in reverse. The iterator yields byte offset positions indicating
    /// the start of each match.
    #[inline]
    fn rfind_iter<'h, 'n, B: ?Sized + AsRef<[u8]>>(
        &'h self,
        needle: &'n B,
    ) -> FindReverse<'h, 'n> {
        FindReverse::new(self.as_bytes(), needle.as_ref())
    }

    /// Returns the index of the first occurrence of the given byte. If the
    /// byte does not occur in this byte string, then `None` is returned.
    #[inline]
    fn find_byte(&self, byte: u8) -> Option<usize> {
        memchr(byte, self.as_bytes())
    }

    /// Returns the index of the last occurrence of the given byte. If the
    /// byte does not occur in this byte string, then `None` is returned.
    #[inline]
    fn rfind_byte(&self, byte: u8) -> Option<usize> {
        memrchr(byte, self.as_bytes())
    }

    /// Returns the index of the first occurrence of the given codepoint.
    /// If the codepoint does not occur in this byte string, then `None` is
    /// returned.
    #[inline]
    fn find_char(&self, ch: char) -> Option<usize> {
        self.find(ch.encode_utf8(&mut [0; 4]))
    }

    /// Returns the index of the last occurrence of the given codepoint.
    /// If the codepoint does not occur in this byte string, then `None` is
    /// returned.
    #[inline]
    fn rfind_char(&self, ch: char) -> Option<usize> {
        self.rfind(ch.encode_utf8(&mut [0; 4]))
    }

    /// Returns the index of the first occurrence of any of the bytes in the
    /// provided set.
    #[inline]
    fn find_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize> {
        byteset::find(self.as_bytes(), byteset.as_ref())
    }

    /// Returns the index of the first occurrence of a byte that is not a
    /// member of the provided set.
    #[inline]
    fn find_not_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize> {
        byteset::find_not(self.as_bytes(), byteset.as_ref())
    }

    /// Returns the index of the last occurrence of any of the bytes in the
    /// provided set.
    #[inline]
    fn rfind_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize> {
        byteset::rfind(self.as_bytes(), byteset.as_ref())
    }

    /// Returns the index of the last occurrence of a byte that is not a member
    /// of the provided set.
    #[inline]
    fn rfind_not_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize> {
        byteset::rfind_not(self.as_bytes(), byteset.as_ref())
    }

    /// Returns an iterator over the fields in a byte string, separated
    /// by contiguous whitespace (according to the Unicode property
    /// `White_Space`).
    #[cfg(feature = "unicode")]
    #[inline]
    fn fields(&self) -> Fields<'_> {
        Fields::new(self.as_bytes())
    }

    /// Returns an iterator over the fields in a byte string, separated by
    /// contiguous codepoints satisfying the given predicate.
    #[inline]
    fn fields_with<F: FnMut(char) -> bool>(&self, f: F) -> FieldsWith<'_, F> {
        FieldsWith::new(self.as_bytes(), f)
    }

    /// Returns an iterator over substrings of this byte string, separated
    /// by the given byte string. Each element yielded is guaranteed not to
    /// include the splitter substring.
    #[inline]
    fn split_str<'h, 's, B: ?Sized + AsRef<[u8]>>(
        &'h self,
        splitter: &'s B,
    ) -> Split<'h, 's> {
        Split::new(self.as_bytes(), splitter.as_ref())
    }

    /// Returns an iterator over substrings of this byte string, separated by
    /// the given byte string, in reverse. Each element yielded is guaranteed
    /// not to include the splitter substring.
    #[inline]
    fn rsplit_str<'h, 's, B: ?Sized + AsRef<[u8]>>(
        &'h self,
        splitter: &'s B,
    ) -> SplitReverse<'h, 's> {
        SplitReverse::new(self.as_bytes(), splitter.as_ref())
    }

    /// Split this byte string at the first occurrence of `splitter`.
    #[inline]
    fn split_once_str<'a, B: ?Sized + AsRef<[u8]>>(
        &'a self,
        splitter: &B,
    ) -> Option<(&'a [u8], &'a [u8])> {
        let bytes = self.as_bytes();
        let splitter = splitter.as_ref();
        let start = Finder::new(splitter).find(bytes)?;
        let end = start + splitter.len();
        Some((&bytes[..start], &bytes[end..]))
    }

    /// Split this byte string at the last occurrence of `splitter`.
    #[inline]
    fn rsplit_once_str<'a, B: ?Sized + AsRef<[u8]>>(
        &'a self,
        splitter: &B,
    ) -> Option<(&'a [u8], &'a [u8])> {
        let bytes = self.as_bytes();
        let splitter = splitter.as_ref();
        let start = FinderReverse::new(splitter).rfind(bytes)?;
        let end = start + splitter.len();
        Some((&bytes[..start], &bytes[end..]))
    }

    /// Returns an iterator of at most `limit` substrings of this byte string,
    /// separated by the given byte string. If `limit` substrings are yielded,
    /// then the last substring will contain the remainder of this byte string.
    #[inline]
    fn splitn_str<'h, 's, B: ?Sized + AsRef<[u8]>>(
        &'h self,
        limit: usize,
        splitter: &'s B,
    ) -> SplitN<'h, 's> {
        SplitN::new(self.as_bytes(), splitter.as_ref(), limit)
    }

    /// Returns an iterator of at most `limit` substrings of this byte string,
    /// separated by the given byte string, in reverse. If `limit` substrings
    /// are yielded, then the last substring will contain the remainder of this
    /// byte string.
    #[inline]
    fn rsplitn_str<'h, 's, B: ?Sized + AsRef<[u8]>>(
        &'h self,
        limit: usize,
        splitter: &'s B,
    ) -> SplitNReverse<'h, 's> {
        SplitNReverse::new(self.as_bytes(), splitter.as_ref(), limit)
    }

    /// Replace all matches of the given needle with the given replacement, and
    /// the result as a new `Vec<u8>`.
    #[inline]
    fn replace<N: AsRef<[u8]>, R: AsRef<[u8]>>(
        &self,
        needle: N,
        replacement: R,
    ) -> Vec<u8> {
        let mut dest = Vec::with_capacity(self.as_bytes().len());
        self.replace_into(needle, replacement, &mut dest);
        dest
    }

    /// Replace up to `limit` matches of the given needle with the given
    /// replacement, and the result as a new `Vec<u8>`.
    #[inline]
    fn replacen<N: AsRef<[u8]>, R: AsRef<[u8]>>(
        &self,
        needle: N,
        replacement: R,
        limit: usize,
    ) -> Vec<u8> {
        let mut dest = Vec::with_capacity(self.as_bytes().len());
        self.replacen_into(needle, replacement, limit, &mut dest);
        dest
    }

    /// Replace all matches of the given needle with the given replacement,
    /// and write the result into the provided `Vec<u8>`.
    #[inline]
    fn replace_into<N: AsRef<[u8]>, R: AsRef<[u8]>>(
        &self,
        needle: N,
        replacement: R,
        dest: &mut Vec<u8>,
    ) {
        let (needle, replacement) = (needle.as_ref(), replacement.as_ref());

        let mut last = 0;
        for start in self.find_iter(needle) {
            dest.push_str(&self.as_bytes()[last..start]);
            dest.push_str(replacement);
            last = start + needle.len();
        }
        dest.push_str(&self.as_bytes()[last..]);
    }

    /// Replace up to `limit` matches of the given needle with the given
    /// replacement, and write the result into the provided `Vec<u8>`.
    #[inline]
    fn replacen_into<N: AsRef<[u8]>, R: AsRef<[u8]>>(
        &self,
        needle: N,
        replacement: R,
        limit: usize,
        dest: &mut Vec<u8>,
    ) {
        let (needle, replacement) = (needle.as_ref(), replacement.as_ref());

        let mut last = 0;
        for start in self.find_iter(needle).take(limit) {
            dest.push_str(&self.as_bytes()[last..start]);
            dest.push_str(replacement);
            last = start + needle.len();
        }
        dest.push_str(&self.as_bytes()[last..]);
    }

    /// Returns an iterator over the bytes in this byte string.
    #[inline]
    fn bytes(&self) -> Bytes<'_> {
        Bytes { it: self.as_bytes().iter() }
    }

    /// Returns an iterator over the Unicode scalar values in this byte string.
    /// If invalid UTF-8 is encountered, then the Unicode replacement codepoint
    /// is yielded instead.
    #[inline]
    fn chars(&self) -> Chars<'_> {
        Chars::new(self.as_bytes())
    }

    /// Returns an iterator over the Unicode scalar values in this byte string
    /// along with their starting and ending byte index positions. If invalid
    /// UTF-8 is encountered, then the Unicode replacement codepoint is yielded
    /// instead.
    #[inline]
    fn char_indices(&self) -> CharIndices<'_> {
        CharIndices::new(self.as_bytes())
    }

    /// Iterate over chunks of valid UTF-8.
    #[inline]
    fn utf8_chunks(&self) -> Utf8Chunks<'_> {
        Utf8Chunks { bytes: self.as_bytes() }
    }

    /// Returns an iterator over the grapheme clusters in this byte string.
    /// If invalid UTF-8 is encountered, then the Unicode replacement codepoint
    /// is yielded instead.
    #[cfg(feature = "unicode")]
    #[inline]
    fn graphemes(&self) -> Graphemes<'_> {
        Graphemes::new(self.as_bytes())
    }

    /// Returns an iterator over the grapheme clusters in this byte string
    /// along with their starting and ending byte index positions. If invalid
    /// UTF-8 is encountered, then the Unicode replacement codepoint is yielded
    /// instead.
    #[cfg(feature = "unicode")]
    #[inline]
    fn grapheme_indices(&self) -> GraphemeIndices<'_> {
        GraphemeIndices::new(self.as_bytes())
    }

    /// Returns an iterator over the words in this byte string. If invalid
    /// UTF-8 is encountered, then the Unicode replacement codepoint is yielded
    /// instead.
    #[cfg(feature = "unicode")]
    #[inline]
    fn words(&self) -> Words<'_> {
        Words::new(self.as_bytes())
    }

    /// Returns an iterator over the words in this byte string along with
    /// their starting and ending byte index positions.
    #[cfg(feature = "unicode")]
    #[inline]
    fn word_indices(&self) -> WordIndices<'_> {
        WordIndices::new(self.as_bytes())
    }

    /// Returns an iterator over the words in this byte string, along with
    /// all breaks between the words. Concatenating all elements yielded by
    /// the iterator results in the original string (modulo Unicode replacement
    /// codepoint substitutions if invalid UTF-8 is encountered).
    #[cfg(feature = "unicode")]
    #[inline]
    fn words_with_breaks(&self) -> WordsWithBreaks<'_> {
        WordsWithBreaks::new(self.as_bytes())
    }

    /// Returns an iterator over the words and their byte offsets in this
    /// byte string, along with all breaks between the words. Concatenating
    /// all elements yielded by the iterator results in the original string
    /// (modulo Unicode replacement codepoint substitutions if invalid UTF-8 is
    /// encountered).
    #[cfg(feature = "unicode")]
    #[inline]
    fn words_with_break_indices(&self) -> WordsWithBreakIndices<'_> {
        WordsWithBreakIndices::new(self.as_bytes())
    }

    /// Returns an iterator over the sentences in this byte string.
    #[cfg(feature = "unicode")]
    #[inline]
    fn sentences(&self) -> Sentences<'_> {
        Sentences::new(self.as_bytes())
    }

    /// Returns an iterator over the sentences in this byte string along with
    /// their starting and ending byte index positions.
    #[cfg(feature = "unicode")]
    #[inline]
    fn sentence_indices(&self) -> SentenceIndices<'_> {
        SentenceIndices::new(self.as_bytes())
    }

    /// An iterator over all lines in a byte string, without their
    /// terminators.
    #[inline]
    fn lines(&self) -> Lines<'_> {
        Lines::new(self.as_bytes())
    }

    /// An iterator over all lines in a byte string, including their
    /// terminators.
    #[inline]
    fn lines_with_terminator(&self) -> LinesWithTerminator<'_> {
        LinesWithTerminator::new(self.as_bytes())
    }

    /// Return a byte string slice with leading and trailing whitespace
    /// removed.
    #[cfg(feature = "unicode")]
    #[inline]
    fn trim(&self) -> &[u8] {
        self.trim_start().trim_end()
    }

    /// Return a byte string slice with leading whitespace removed.
    #[cfg(feature = "unicode")]
    #[inline]
    fn trim_start(&self) -> &[u8] {
        let start = whitespace_len_fwd(self.as_bytes());
        &self.as_bytes()[start..]
    }

    /// Return a byte string slice with trailing whitespace removed.
    #[cfg(feature = "unicode")]
    #[inline]
    fn trim_end(&self) -> &[u8] {
        let end = whitespace_len_rev(self.as_bytes());
        &self.as_bytes()[..end]
    }

    /// Return a byte string slice with leading and trailing characters
    /// satisfying the given predicate removed.
    #[inline]
    fn trim_with<F: FnMut(char) -> bool>(&self, mut trim: F) -> &[u8] {
        self.trim_start_with(&mut trim).trim_end_with(&mut trim)
    }

    /// Return a byte string slice with leading characters satisfying the given
    /// predicate removed.
    #[inline]
    fn trim_start_with<F: FnMut(char) -> bool>(&self, mut trim: F) -> &[u8] {
        for (s, _, ch) in self.char_indices() {
            if !trim(ch) {
                return &self.as_bytes()[s..];
            }
        }
        b""
    }

    /// Return a byte string slice with trailing characters satisfying the
    /// given predicate removed.
    #[inline]
    fn trim_end_with<F: FnMut(char) -> bool>(&self, mut trim: F) -> &[u8] {
        for (_, e, ch) in self.char_indices().rev() {
            if !trim(ch) {
                return &self.as_bytes()[..e];
            }
        }
        b""
    }

    /// Returns a new `Vec<u8>` containing the lowercase equivalent of this
    /// byte string.
    #[cfg(all(feature = "alloc", feature = "unicode"))]
    #[inline]
    fn to_lowercase(&self) -> Vec<u8> {
        let mut buf = vec![];
        self.to_lowercase_into(&mut buf);
        buf
    }

    /// Writes the lowercase equivalent of this byte string into the given
    /// buffer. The buffer is not cleared before written to.
    #[cfg(all(feature = "alloc", feature = "unicode"))]
    #[inline]
    fn to_lowercase_into(&self, buf: &mut Vec<u8>) {
        // TODO: This is the best we can do given what std exposes I think.
        // If we roll our own case handling, then we might be able to do this
        // a bit faster. We shouldn't roll our own case handling unless we
        // need to, e.g., for doing caseless matching or case folding.

        // TODO(BUG): This doesn't handle any special casing rules.

        buf.reserve(self.as_bytes().len());
        for (s, e, ch) in self.char_indices() {
            if ch == '\u{FFFD}' {
                buf.push_str(&self.as_bytes()[s..e]);
            } else if ch.is_ascii() {
                buf.push_char(ch.to_ascii_lowercase());
            } else {
                for upper in ch.to_lowercase() {
                    buf.push_char(upper);
                }
            }
        }
    }

    /// Returns a new `Vec<u8>` containing the ASCII lowercase equivalent of
    /// this byte string.
    #[inline]
    fn to_ascii_lowercase(&self) -> Vec<u8> {
        self.as_bytes().to_ascii_lowercase()
    }

    /// Convert this byte string to its lowercase ASCII equivalent in place.
    #[inline]
    fn make_ascii_lowercase(&mut self) {
        self.as_bytes_mut().make_ascii_lowercase();
    }

    /// Returns a new `Vec<u8>` containing the uppercase equivalent of this
    /// byte string.
    #[cfg(all(feature = "alloc", feature = "unicode"))]
    #[inline]
    fn to_uppercase(&self) -> Vec<u8> {
        let mut buf = vec![];
        self.to_uppercase_into(&mut buf);
        buf
    }

    /// Writes the uppercase equivalent of this byte string into the given
    /// buffer. The buffer is not cleared before written to.
    #[cfg(all(feature = "alloc", feature = "unicode"))]
    #[inline]
    fn to_uppercase_into(&self, buf: &mut Vec<u8>) {
        // TODO: This is the best we can do given what std exposes I think.
        // If we roll our own case handling, then we might be able to do this
        // a bit faster. We shouldn't roll our own case handling unless we
        // need to, e.g., for doing caseless matching or case folding.
        buf.reserve(self.as_bytes().len());
        for (s, e, ch) in self.char_indices() {
            if ch == '\u{FFFD}' {
                buf.push_str(&self.as_bytes()[s..e]);
            } else if ch.is_ascii() {
                buf.push_char(ch.to_ascii_uppercase());
            } else {
                for upper in ch.to_uppercase() {
                    buf.push_char(upper);
                }
            }
        }
    }

    /// Returns a new `Vec<u8>` containing the ASCII uppercase equivalent of
    /// this byte string.
    #[inline]
    fn to_ascii_uppercase(&self) -> Vec<u8> {
        self.as_bytes().to_ascii_uppercase()
    }

    /// Convert this byte string to its uppercase ASCII equivalent in place.
    #[inline]
    fn make_ascii_uppercase(&mut self) {
        self.as_bytes_mut().make_ascii_uppercase();
    }

    /// Escapes this byte string into a sequence of `char` values.
    #[inline]
    fn escape_bytes(&self) -> EscapeBytes<'_> {
        EscapeBytes::new(self.as_bytes())
    }

    /// Reverse the bytes in this string, in place.
    #[inline]
    fn reverse_bytes(&mut self) {
        self.as_bytes_mut().reverse();
    }

    /// Reverse the codepoints in this string, in place.
    #[inline]
    fn reverse_chars(&mut self) {
        let mut i = 0;
        loop {
            let (_, size) = utf8::decode(&self.as_bytes()[i..]);
            if size == 0 {
                break;
            }
            if size > 1 {
                self.as_bytes_mut()[i..i + size].reverse_bytes();
            }
            i += size;
        }
        self.reverse_bytes();
    }

    /// Reverse the graphemes in this string, in place.
    #[cfg(feature = "unicode")]
    #[inline]
    fn reverse_graphemes(&mut self) {
        use crate::unicode::decode_grapheme;

        let mut i = 0;
        loop {
            let (_, size) = decode_grapheme(&self.as_bytes()[i..]);
            if size == 0 {
                break;
            }
            if size > 1 {
                self.as_bytes_mut()[i..i + size].reverse_bytes();
            }
            i += size;
        }
        self.reverse_bytes();
    }

    /// Returns true if and only if every byte in this byte string is ASCII.
    #[inline]
    fn is_ascii(&self) -> bool {
        ascii::first_non_ascii_byte(self.as_bytes()) == self.as_bytes().len()
    }

    /// Returns true if and only if the entire byte string is valid UTF-8.
    #[inline]
    fn is_utf8(&self) -> bool {
        utf8::validate(self.as_bytes()).is_ok()
    }

    /// Returns the last byte in this byte string, if it's non-empty. If this
    /// byte string is empty, this returns `None`.
    #[inline]
    fn last_byte(&self) -> Option<u8> {
        let bytes = self.as_bytes();
        bytes.get(bytes.len().saturating_sub(1)).map(|&b| b)
    }

    /// Returns the index of the first non-ASCII byte in this byte string (if
    /// any such indices exist). Specifically, it returns the index of the
    /// first byte with a value greater than or equal to `0x80`.
    #[inline]
    fn find_non_ascii_byte(&self) -> Option<usize> {
        let index = ascii::first_non_ascii_byte(self.as_bytes());
        if index == self.as_bytes().len() {
            None
        } else {
            Some(index)
        }
    }
}

/// A single substring searcher fixed to a particular needle.
#[derive(Clone, Debug)]
pub struct Finder<'a>(memmem::Finder<'a>);

impl<'a> Finder<'a> {
    /// Create a new finder for the given needle.
    #[inline]
    pub fn new<B: ?Sized + AsRef<[u8]>>(needle: &'a B) -> Finder<'a> {
        Finder(memmem::Finder::new(needle.as_ref()))
    }

    /// Convert this finder into its owned variant, such that it no longer
    /// borrows the needle.
    #[inline]
    pub fn into_owned(self) -> Finder<'static> {
        Finder(self.0.into_owned())
    }

    /// Returns the needle that this finder searches for.
    ///
    /// Note that the lifetime of the needle returned is tied to the lifetime
    /// of the finder, and may be shorter than the `'a` lifetime. Namely, a
    /// finder's needle can be either borrowed or owned, so the lifetime of the
    /// needle returned must necessarily be the shorter of the two.
    #[inline]
    pub fn needle(&self) -> &[u8] {
        self.0.needle()
    }

    /// Returns the index of the first occurrence of this needle in the given
    /// haystack.
    #[inline]
    pub fn find<B: AsRef<[u8]>>(&self, haystack: B) -> Option<usize> {
        self.0.find(haystack.as_ref())
    }
}

/// A single substring reverse searcher fixed to a particular needle.
///
/// The purpose of this type is to permit callers to construct a substring
/// searcher that can be used to search haystacks without the overhead of
/// constructing the searcher in the first place. This is a somewhat niche
/// concern when it's necessary to re-use the same needle to search multiple
/// different haystacks with as little overhead as possible. In general, using
/// [`ByteSlice::rfind`](trait.ByteSlice.html#method.rfind)
/// or
/// [`ByteSlice::rfind_iter`](trait.ByteSlice.html#method.rfind_iter)
/// is good enough, but `FinderReverse` is useful when you can meaningfully
/// observe searcher construction time in a profile.
///
/// When the `std` feature is enabled, then this type has an `into_owned`
/// version which permits building a `FinderReverse` that is not connected to
/// the lifetime of its needle.
#[derive(Clone, Debug)]
pub struct FinderReverse<'a>(memmem::FinderRev<'a>);

impl<'a> FinderReverse<'a> {
    /// Create a new reverse finder for the given needle.
    #[inline]
    pub fn new<B: ?Sized + AsRef<[u8]>>(needle: &'a B) -> FinderReverse<'a> {
        FinderReverse(memmem::FinderRev::new(needle.as_ref()))
    }

    /// Convert this finder into its owned variant, such that it no longer
    /// borrows the needle.
    #[inline]
    pub fn into_owned(self) -> FinderReverse<'static> {
        FinderReverse(self.0.into_owned())
    }

    /// Returns the needle that this finder searches for.
    ///
    /// Note that the lifetime of the needle returned is tied to the lifetime
    /// of this finder, and may be shorter than the `'a` lifetime. Namely,
    /// a finder's needle can be either borrowed or owned, so the lifetime of
    /// the needle returned must necessarily be the shorter of the two.
    #[inline]
    pub fn needle(&self) -> &[u8] {
        self.0.needle()
    }

    /// Returns the index of the last occurrence of this needle in the given
    /// haystack.
    #[inline]
    pub fn rfind<B: AsRef<[u8]>>(&self, haystack: B) -> Option<usize> {
        self.0.rfind(haystack.as_ref())
    }
}

/// An iterator over non-overlapping substring matches.
///
/// Matches are reported by the byte offset at which they begin.
///
/// `'h` is the lifetime of the haystack while `'n` is the lifetime of the
/// needle.
#[derive(Clone, Debug)]
pub struct Find<'h, 'n> {
    it: memmem::FindIter<'h, 'n>,
    haystack: &'h [u8],
    needle: &'n [u8],
}

impl<'h, 'n> Find<'h, 'n> {
    fn new(haystack: &'h [u8], needle: &'n [u8]) -> Find<'h, 'n> {
        Find { it: memmem::find_iter(haystack, needle), haystack, needle }
    }
}

impl<'h, 'n> Iterator for Find<'h, 'n> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        self.it.next()
    }
}

/// An iterator over non-overlapping substring matches in reverse.
///
/// Matches are reported by the byte offset at which they begin.
///
/// `'h` is the lifetime of the haystack while `'n` is the lifetime of the
/// needle.
#[derive(Clone, Debug)]
pub struct FindReverse<'h, 'n> {
    it: memmem::FindRevIter<'h, 'n>,
    haystack: &'h [u8],
    needle: &'n [u8],
}

impl<'h, 'n> FindReverse<'h, 'n> {
    fn new(haystack: &'h [u8], needle: &'n [u8]) -> FindReverse<'h, 'n> {
        FindReverse {
            it: memmem::rfind_iter(haystack, needle),
            haystack,
            needle,
        }
    }

    fn haystack(&self) -> &'h [u8] {
        self.haystack
    }

    fn needle(&self) -> &'n [u8] {
        self.needle
    }
}

impl<'h, 'n> Iterator for FindReverse<'h, 'n> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        self.it.next()
    }
}

/// An iterator over the bytes in a byte string.
///
/// `'a` is the lifetime of the byte string being traversed.
#[derive(Clone, Debug)]
pub struct Bytes<'a> {
    it: slice::Iter<'a, u8>,
}

impl<'a> Bytes<'a> {
    /// Views the remaining underlying data as a subslice of the original data.
    /// This has the same lifetime as the original slice,
    /// and so the iterator can continue to be used while this exists.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.it.as_slice()
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        self.it.next().map(|&b| b)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Bytes<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<u8> {
        self.it.next_back().map(|&b| b)
    }
}

impl<'a> ExactSizeIterator for Bytes<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.it.len()
    }
}

impl<'a> iter::FusedIterator for Bytes<'a> {}

/// An iterator over the fields in a byte string, separated by whitespace.
///
/// Whitespace for this iterator is defined by the Unicode property
/// `White_Space`.
///
/// This iterator splits on contiguous runs of whitespace, such that the fields
/// in `foo\t\t\n  \nbar` are `foo` and `bar`.
///
/// `'a` is the lifetime of the byte string being split.
#[cfg(feature = "unicode")]
#[derive(Clone, Debug)]
pub struct Fields<'a> {
    it: FieldsWith<'a, fn(char) -> bool>,
}

#[cfg(feature = "unicode")]
impl<'a> Fields<'a> {
    fn new(bytes: &'a [u8]) -> Fields<'a> {
        Fields { it: bytes.fields_with(|ch| ch.is_whitespace()) }
    }
}

#[cfg(feature = "unicode")]
impl<'a> Iterator for Fields<'a> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        self.it.next()
    }
}

/// An iterator over fields in the byte string, separated by a predicate over
/// codepoints.
///
/// This iterator splits a byte string based on its predicate function such
/// that the elements returned are separated by contiguous runs of codepoints
/// for which the predicate returns true.
///
/// `'a` is the lifetime of the byte string being split, while `F` is the type
/// of the predicate, i.e., `FnMut(char) -> bool`.
#[derive(Clone, Debug)]
pub struct FieldsWith<'a, F> {
    f: F,
    bytes: &'a [u8],
    chars: CharIndices<'a>,
}

impl<'a, F: FnMut(char) -> bool> FieldsWith<'a, F> {
    fn new(bytes: &'a [u8], f: F) -> FieldsWith<'a, F> {
        FieldsWith { f, bytes, chars: bytes.char_indices() }
    }
}

impl<'a, F: FnMut(char) -> bool> Iterator for FieldsWith<'a, F> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        let (start, mut end);
        loop {
            match self.chars.next() {
                None => return None,
                Some((s, e, ch)) => {
                    if !(self.f)(ch) {
                        start = s;
                        end = e;
                        break;
                    }
                }
            }
        }
        while let Some((_, e, ch)) = self.chars.next() {
            if (self.f)(ch) {
                break;
            }
            end = e;
        }
        Some(&self.bytes[start..end])
    }
}

/// An iterator over substrings in a byte string, split by a separator.
///
/// `'h` is the lifetime of the byte string being split (the haystack), while
/// `'s` is the lifetime of the byte string doing the splitting.
#[derive(Clone, Debug)]
pub struct Split<'h, 's> {
    finder: Find<'h, 's>,
    /// The end position of the previous match of our splitter. The element
    /// we yield corresponds to the substring starting at `last` up to the
    /// beginning of the next match of the splitter.
    last: usize,
    /// Only set when iteration is complete. A corner case here is when a
    /// splitter is matched at the end of the haystack. At that point, we still
    /// need to yield an empty string following it.
    done: bool,
}

impl<'h, 's> Split<'h, 's> {
    fn new(haystack: &'h [u8], splitter: &'s [u8]) -> Split<'h, 's> {
        let finder = haystack.find_iter(splitter);
        Split { finder, last: 0, done: false }
    }
}

impl<'h, 's> Iterator for Split<'h, 's> {
    type Item = &'h [u8];

    #[inline]
    fn next(&mut self) -> Option<&'h [u8]> {
        let haystack = self.finder.haystack;
        match self.finder.next() {
            Some(start) => {
                let next = &haystack[self.last..start];
                self.last = start + self.finder.needle.len();
                Some(next)
            }
            None => {
                if self.last >= haystack.len() {
                    if !self.done {
                        self.done = true;
                        Some(b"")
                    } else {
                        None
                    }
                } else {
                    let s = &haystack[self.last..];
                    self.last = haystack.len();
                    self.done = true;
                    Some(s)
                }
            }
        }
    }
}

/// An iterator over substrings in a byte string, split by a separator, in
/// reverse.
///
/// `'h` is the lifetime of the byte string being split (the haystack), while
/// `'s` is the lifetime of the byte string doing the splitting.
#[derive(Clone, Debug)]
pub struct SplitReverse<'h, 's> {
    finder: FindReverse<'h, 's>,
    /// The end position of the previous match of our splitter. The element
    /// we yield corresponds to the substring starting at `last` up to the
    /// beginning of the next match of the splitter.
    last: usize,
    /// Only set when iteration is complete. A corner case here is when a
    /// splitter is matched at the end of the haystack. At that point, we still
    /// need to yield an empty string following it.
    done: bool,
}

impl<'h, 's> SplitReverse<'h, 's> {
    fn new(haystack: &'h [u8], splitter: &'s [u8]) -> SplitReverse<'h, 's> {
        let finder = haystack.rfind_iter(splitter);
        SplitReverse { finder, last: haystack.len(), done: false }
    }
}

impl<'h, 's> Iterator for SplitReverse<'h, 's> {
    type Item = &'h [u8];

    #[inline]
    fn next(&mut self) -> Option<&'h [u8]> {
        let haystack = self.finder.haystack();
        match self.finder.next() {
            Some(start) => {
                let nlen = self.finder.needle().len();
                let next = &haystack[start + nlen..self.last];
                self.last = start;
                Some(next)
            }
            None => {
                if self.last == 0 {
                    if !self.done {
                        self.done = true;
                        Some(b"")
                    } else {
                        None
                    }
                } else {
                    let s = &haystack[..self.last];
                    self.last = 0;
                    self.done = true;
                    Some(s)
                }
            }
        }
    }
}

/// An iterator over at most `n` substrings in a byte string, split by a
/// separator.
///
/// `'h` is the lifetime of the byte string being split (the haystack), while
/// `'s` is the lifetime of the byte string doing the splitting.
#[derive(Clone, Debug)]
pub struct SplitN<'h, 's> {
    split: Split<'h, 's>,
    limit: usize,
    count: usize,
}

impl<'h, 's> SplitN<'h, 's> {
    fn new(
        haystack: &'h [u8],
        splitter: &'s [u8],
        limit: usize,
    ) -> SplitN<'h, 's> {
        let split = haystack.split_str(splitter);
        SplitN { split, limit, count: 0 }
    }
}

impl<'h, 's> Iterator for SplitN<'h, 's> {
    type Item = &'h [u8];

    #[inline]
    fn next(&mut self) -> Option<&'h [u8]> {
        self.count += 1;
        if self.count > self.limit || self.split.done {
            None
        } else if self.count == self.limit {
            Some(&self.split.finder.haystack[self.split.last..])
        } else {
            self.split.next()
        }
    }
}

/// An iterator over at most `n` substrings in a byte string, split by a
/// separator, in reverse.
///
/// `'h` is the lifetime of the byte string being split (the haystack), while
/// `'s` is the lifetime of the byte string doing the splitting.
#[derive(Clone, Debug)]
pub struct SplitNReverse<'h, 's> {
    split: SplitReverse<'h, 's>,
    limit: usize,
    count: usize,
}

impl<'h, 's> SplitNReverse<'h, 's> {
    fn new(
        haystack: &'h [u8],
        splitter: &'s [u8],
        limit: usize,
    ) -> SplitNReverse<'h, 's> {
        let split = haystack.rsplit_str(splitter);
        SplitNReverse { split, limit, count: 0 }
    }
}

impl<'h, 's> Iterator for SplitNReverse<'h, 's> {
    type Item = &'h [u8];

    #[inline]
    fn next(&mut self) -> Option<&'h [u8]> {
        self.count += 1;
        if self.count > self.limit || self.split.done {
            None
        } else if self.count == self.limit {
            Some(&self.split.finder.haystack()[..self.split.last])
        } else {
            self.split.next()
        }
    }
}

/// An iterator over all lines in a byte string, without their terminators.
///
/// For this iterator, the only line terminators recognized are `\r\n` and
/// `\n`.
///
/// `'a` is the lifetime of the byte string being iterated over.
#[derive(Clone, Debug)]
pub struct Lines<'a> {
    it: LinesWithTerminator<'a>,
}

impl<'a> Lines<'a> {
    fn new(bytes: &'a [u8]) -> Lines<'a> {
        Lines { it: LinesWithTerminator::new(bytes) }
    }

    /// Return a copy of the rest of the underlying bytes without affecting the
    /// iterator itself.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.it.bytes
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        Some(trim_last_terminator(self.it.next()?))
    }
}

impl<'a> DoubleEndedIterator for Lines<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(trim_last_terminator(self.it.next_back()?))
    }
}

impl<'a> iter::FusedIterator for Lines<'a> {}

/// An iterator over all lines in a byte string, including their terminators.
///
/// For this iterator, the only line terminator recognized is `\n`. (Since
/// line terminators are included, this also handles `\r\n` line endings.)
///
/// Line terminators are only included if they are present in the original
/// byte string. For example, the last line in a byte string may not end with
/// a line terminator.
///
/// Concatenating all elements yielded by this iterator is guaranteed to yield
/// the original byte string.
///
/// `'a` is the lifetime of the byte string being iterated over.
#[derive(Clone, Debug)]
pub struct LinesWithTerminator<'a> {
    bytes: &'a [u8],
}

impl<'a> LinesWithTerminator<'a> {
    fn new(bytes: &'a [u8]) -> LinesWithTerminator<'a> {
        LinesWithTerminator { bytes }
    }

    /// Return a copy of the rest of the underlying bytes without affecting the
    /// iterator itself.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a> Iterator for LinesWithTerminator<'a> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        match self.bytes.find_byte(b'\n') {
            None if self.bytes.is_empty() => None,
            None => {
                let line = self.bytes;
                self.bytes = b"";
                Some(line)
            }
            Some(end) => {
                let line = &self.bytes[..end + 1];
                self.bytes = &self.bytes[end + 1..];
                Some(line)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for LinesWithTerminator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let end = self.bytes.len().checked_sub(1)?;
        match self.bytes[..end].rfind_byte(b'\n') {
            None => {
                let line = self.bytes;
                self.bytes = b"";
                Some(line)
            }
            Some(end) => {
                let line = &self.bytes[end + 1..];
                self.bytes = &self.bytes[..end + 1];
                Some(line)
            }
        }
    }
}

impl<'a> iter::FusedIterator for LinesWithTerminator<'a> {}

/// 去掉结尾的\r\n
fn trim_last_terminator(mut s: &[u8]) -> &[u8] {
    if s.last_byte() == Some(b'\n') {
        s = &s[..s.len() - 1];
        if s.last_byte() == Some(b'\r') {
            s = &s[..s.len() - 1];
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use std::path::Path;

    /// &OsStr -> Option<&[u8]>
    #[test]
    fn test_from_os_str() {    
        let os_str = OsStr::new("foo");
        let bs = <[u8]>::from_os_str(os_str).expect("should be valid UTF-8");
        assert_eq!(bs, B("foo"));
    }

    /// &Path -> Option<&[u8]>
    #[test]
    fn test_from_path() {
     let path = Path::new("foo");
     let bs = <[u8]>::from_path(path).expect("should be valid UTF-8");
     assert_eq!(bs, B("foo"));
    }

    #[test]
    fn test_to_str() {
        let s = B("☃βツ").to_str().unwrap();
        assert_eq!("☃βツ", s);
        
        let mut bstring = <Vec<u8>>::from("☃βツ");
        // 字节数组中添加一个字节
        bstring.push(b'\xFF');
        let err = bstring.to_str().unwrap_err();
        assert_eq!(8, err.valid_up_to());
    }

    #[test]
    fn test_to_os_str() {
        let os_str = b"foo".to_os_str().expect("should be valid UTF-8");
        assert_eq!(os_str, "foo");
    }
}
use super::{error::Utf8Error, ext_slice::ByteSlice};
use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
use core::mem;
use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt, ops,
};

/// A wrapper for `&[u8]` that provides convenient string oriented trait impls.
#[derive(Hash)]
#[repr(transparent)]
pub struct BStr {
    pub(crate) bytes: [u8],
}

impl BStr {
    /// Directly creates a `BStr` slice from anything that can be converted
    /// to a byte slice.
    #[inline]
    pub fn new<'a, B: ?Sized + AsRef<[u8]>>(bytes: &'a B) -> &'a BStr {
        BStr::from_bytes(bytes.as_ref())
    }

    #[inline]
    pub(crate) fn new_mut<B: ?Sized + AsMut<[u8]>>(
        bytes: &mut B,
    ) -> &mut BStr {
        BStr::from_bytes_mut(bytes.as_mut())
    }

    #[inline]
    pub(crate) fn from_bytes(slice: &[u8]) -> &BStr {
        unsafe { mem::transmute(slice) }
    }

    #[inline]
    pub(crate) fn from_bytes_mut(slice: &mut [u8]) -> &mut BStr {
        unsafe { mem::transmute(slice) }
    }

    #[inline]
    pub(crate) fn from_boxed_bytes(slice: Box<[u8]>) -> Box<BStr> {
        unsafe { Box::from_raw(Box::into_raw(slice) as _) }
    }

    #[inline]
    pub(crate) fn into_boxed_bytes(slice: Box<BStr>) -> Box<[u8]> {
        unsafe { Box::from_raw(Box::into_raw(slice) as _) }
    }

    #[inline]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Display for BStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /// Write the given bstr (lossily) to the given formatter.
        fn write_bstr(
            f: &mut fmt::Formatter<'_>,
            bstr: &BStr,
        ) -> Result<(), fmt::Error> {
            for chunk in bstr.utf8_chunks() {
                f.write_str(chunk.valid())?;
                if !chunk.invalid().is_empty() {
                    f.write_str("\u{FFFD}")?;
                }
            }
            Ok(())
        }

        /// Write 'num' fill characters to the given formatter.
        fn write_pads(f: &mut fmt::Formatter<'_>, num: usize) -> fmt::Result {
            let fill = f.fill();
            for _ in 0..num {
                f.write_fmt(format_args!("{}", fill))?;
            }
            Ok(())
        }

        if let Some(align) = f.align() {
            let width = f.width().unwrap_or(0);
            let nchars = self.chars().count();
            let remaining_pads = width.saturating_sub(nchars);
            match align {
                fmt::Alignment::Left => {
                    write_bstr(f, self)?;
                    write_pads(f, remaining_pads)?;
                }
                fmt::Alignment::Right => {
                    write_pads(f, remaining_pads)?;
                    write_bstr(f, self)?;
                }
                fmt::Alignment::Center => {
                    let half = remaining_pads / 2;
                    let second_half =
                        if remaining_pads % 2 == 0 { half } else { half + 1 };
                    write_pads(f, half)?;
                    write_bstr(f, self)?;
                    write_pads(f, second_half)?;
                }
            }
            Ok(())
        } else {
            write_bstr(f, self)?;
            Ok(())
        }
    }
}

impl fmt::Debug for BStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for (s, e, ch) in self.char_indices() {
            match ch {
                '\0' => write!(f, "\\0")?,
                '\u{FFFD}' => {
                    let bytes = self[s..e].as_bytes();
                    if bytes == b"\xEF\xBF\xBD" {
                        write!(f, "{}", ch.escape_debug())?;
                    } else {
                        for &b in self[s..e].as_bytes() {
                            write!(f, r"\x{:02X}", b)?;
                        }
                    }
                }
                // ASCII control characters except \0, \n, \r, \t
                '\x01'..='\x08'
                | '\x0b'
                | '\x0c'
                | '\x0e'..='\x19'
                | '\x7f' => {
                    write!(f, "\\x{:02x}", ch as u32)?;
                }
                '\n' | '\r' | '\t' | _ => {
                    write!(f, "{}", ch.escape_debug())?;
                }
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}

impl ops::Deref for BStr {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}

impl ops::DerefMut for BStr {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl ops::Index<usize> for BStr {
    type Output = u8;

    #[inline]
    fn index(&self, idx: usize) -> &u8 {
        &self.as_bytes()[idx]
    }
}

impl ops::Index<ops::RangeFull> for BStr {
    type Output = BStr;

    #[inline]
    fn index(&self, _: ops::RangeFull) -> &BStr {
        self
    }
}

impl ops::Index<ops::Range<usize>> for BStr {
    type Output = BStr;

    #[inline]
    fn index(&self, r: ops::Range<usize>) -> &BStr {
        BStr::new(&self.as_bytes()[r.start..r.end])
    }
}

impl ops::Index<ops::RangeInclusive<usize>> for BStr {
    type Output = BStr;

    #[inline]
    fn index(&self, r: ops::RangeInclusive<usize>) -> &BStr {
        BStr::new(&self.as_bytes()[*r.start()..=*r.end()])
    }
}

impl ops::Index<ops::RangeFrom<usize>> for BStr {
    type Output = BStr;

    #[inline]
    fn index(&self, r: ops::RangeFrom<usize>) -> &BStr {
        BStr::new(&self.as_bytes()[r.start..])
    }
}

impl ops::Index<ops::RangeTo<usize>> for BStr {
    type Output = BStr;

    #[inline]
    fn index(&self, r: ops::RangeTo<usize>) -> &BStr {
        BStr::new(&self.as_bytes()[..r.end])
    }
}

impl ops::Index<ops::RangeToInclusive<usize>> for BStr {
    type Output = BStr;

    #[inline]
    fn index(&self, r: ops::RangeToInclusive<usize>) -> &BStr {
        BStr::new(&self.as_bytes()[..=r.end])
    }
}

impl ops::IndexMut<usize> for BStr {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        &mut self.bytes[idx]
    }
}

impl ops::IndexMut<ops::RangeFull> for BStr {
    #[inline]
    fn index_mut(&mut self, _: ops::RangeFull) -> &mut BStr {
        self
    }
}

impl ops::IndexMut<ops::Range<usize>> for BStr {
    #[inline]
    fn index_mut(&mut self, r: ops::Range<usize>) -> &mut BStr {
        BStr::from_bytes_mut(&mut self.bytes[r.start..r.end])
    }
}

impl ops::IndexMut<ops::RangeInclusive<usize>> for BStr {
    #[inline]
    fn index_mut(&mut self, r: ops::RangeInclusive<usize>) -> &mut BStr {
        BStr::from_bytes_mut(&mut self.bytes[*r.start()..=*r.end()])
    }
}

impl ops::IndexMut<ops::RangeFrom<usize>> for BStr {
    #[inline]
    fn index_mut(&mut self, r: ops::RangeFrom<usize>) -> &mut BStr {
        BStr::from_bytes_mut(&mut self.bytes[r.start..])
    }
}

impl ops::IndexMut<ops::RangeTo<usize>> for BStr {
    #[inline]
    fn index_mut(&mut self, r: ops::RangeTo<usize>) -> &mut BStr {
        BStr::from_bytes_mut(&mut self.bytes[..r.end])
    }
}

impl ops::IndexMut<ops::RangeToInclusive<usize>> for BStr {
    #[inline]
    fn index_mut(&mut self, r: ops::RangeToInclusive<usize>) -> &mut BStr {
        BStr::from_bytes_mut(&mut self.bytes[..=r.end])
    }
}

impl AsRef<[u8]> for BStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<BStr> for BStr {
    #[inline]
    fn as_ref(&self) -> &BStr {
        self
    }
}

impl AsRef<BStr> for [u8] {
    #[inline]
    fn as_ref(&self) -> &BStr {
        BStr::new(self)
    }
}

impl AsRef<BStr> for str {
    #[inline]
    fn as_ref(&self) -> &BStr {
        BStr::new(self)
    }
}

impl AsMut<[u8]> for BStr {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl AsMut<BStr> for [u8] {
    #[inline]
    fn as_mut(&mut self) -> &mut BStr {
        BStr::new_mut(self)
    }
}

impl Borrow<BStr> for [u8] {
    #[inline]
    fn borrow(&self) -> &BStr {
        self.as_bstr()
    }
}

impl Borrow<BStr> for str {
    #[inline]
    fn borrow(&self) -> &BStr {
        self.as_bytes().as_bstr()
    }
}

impl Borrow<[u8]> for BStr {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl BorrowMut<BStr> for [u8] {
    #[inline]
    fn borrow_mut(&mut self) -> &mut BStr {
        BStr::new_mut(self)
    }
}

impl BorrowMut<[u8]> for BStr {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl<'a> Default for &'a BStr {
    fn default() -> &'a BStr {
        BStr::from_bytes(b"")
    }
}

impl<'a> Default for &'a mut BStr {
    fn default() -> &'a mut BStr {
        BStr::from_bytes_mut(&mut [])
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for &'a BStr {
    #[inline]
    fn from(s: &'a [u8; N]) -> &'a BStr {
        BStr::from_bytes(s)
    }
}

impl<'a> From<&'a [u8]> for &'a BStr {
    #[inline]
    fn from(s: &'a [u8]) -> &'a BStr {
        BStr::from_bytes(s)
    }
}

impl<'a> From<&'a BStr> for &'a [u8] {
    #[inline]
    fn from(s: &'a BStr) -> &'a [u8] {
        BStr::as_bytes(s)
    }
}

impl<'a> From<&'a str> for &'a BStr {
    #[inline]
    fn from(s: &'a str) -> &'a BStr {
        BStr::from_bytes(s.as_bytes())
    }
}

impl<'a> From<&'a BStr> for Cow<'a, BStr> {
    #[inline]
    fn from(s: &'a BStr) -> Cow<'a, BStr> {
        Cow::Borrowed(s)
    }
}

impl From<Box<[u8]>> for Box<BStr> {
    #[inline]
    fn from(s: Box<[u8]>) -> Box<BStr> {
        BStr::from_boxed_bytes(s)
    }
}

impl From<Box<BStr>> for Box<[u8]> {
    #[inline]
    fn from(s: Box<BStr>) -> Box<[u8]> {
        BStr::into_boxed_bytes(s)
    }
}

impl<'a> TryFrom<&'a BStr> for &'a str {
    type Error = Utf8Error;

    #[inline]
    fn try_from(s: &'a BStr) -> Result<&'a str, Utf8Error> {
        s.as_bytes().to_str()
    }
}

impl<'a> TryFrom<&'a BStr> for String {
    type Error = Utf8Error;

    #[inline]
    fn try_from(s: &'a BStr) -> Result<String, Utf8Error> {
        Ok(s.as_bytes().to_str()?.into())
    }
}

impl Clone for Box<BStr> {
    #[inline]
    fn clone(&self) -> Self {
        BStr::from_boxed_bytes(self.as_bytes().into())
    }
}

impl Eq for BStr {}

impl PartialEq<BStr> for BStr {
    #[inline]
    fn eq(&self, other: &BStr) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl_partial_eq!(BStr, [u8]);
impl_partial_eq!(BStr, &'a [u8]);
impl_partial_eq!(BStr, str);
impl_partial_eq!(BStr, &'a str);

impl_partial_eq!(BStr, Vec<u8>);
impl_partial_eq!(&'a BStr, Vec<u8>);
impl_partial_eq!(BStr, String);
impl_partial_eq!(&'a BStr, String);
impl_partial_eq_cow!(&'a BStr, Cow<'a, BStr>);
impl_partial_eq_cow!(&'a BStr, Cow<'a, str>);
impl_partial_eq_cow!(&'a BStr, Cow<'a, [u8]>);

impl PartialOrd for BStr {
    #[inline]
    fn partial_cmp(&self, other: &BStr) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.as_bytes(), other.as_bytes())
    }
}

impl Ord for BStr {
    #[inline]
    fn cmp(&self, other: &BStr) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl_partial_ord!(BStr, [u8]);
impl_partial_ord!(BStr, &'a [u8]);
impl_partial_ord!(BStr, str);
impl_partial_ord!(BStr, &'a str);

impl_partial_ord!(BStr, Vec<u8>);
impl_partial_ord!(&'a BStr, Vec<u8>);
impl_partial_ord!(BStr, String);
impl_partial_ord!(&'a BStr, String);

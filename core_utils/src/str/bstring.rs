use core::{cmp::Ordering, fmt, ops, str::FromStr};
use alloc::{
    borrow::{Borrow, BorrowMut, Cow, ToOwned},
    string::String,
    vec,
    vec::Vec,
};

use super::{
    bstr::BStr, ext_slice::ByteSlice, ext_vec::ByteVec, error::{FromUtf8Error, Utf8Error}
};


/// A wrapper for `Vec<u8>` that provides convenient string oriented trait
/// impls.
#[derive(Clone, Hash)]
pub struct BString {
    bytes: Vec<u8>,
}

impl BString {
    /// Constructs a new `BString` from the given [`Vec`].
    #[inline]
    pub const fn new(bytes: Vec<u8>) -> BString {
        BString { bytes }
    }

    #[inline]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    pub(crate) fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    #[inline]
    pub(crate) fn as_bstr(&self) -> &BStr {
        BStr::new(&self.bytes)
    }

    #[inline]
    pub(crate) fn as_mut_bstr(&mut self) -> &mut BStr {
        BStr::new_mut(&mut self.bytes)
    }

    #[inline]
    pub(crate) fn as_vec(&self) -> &Vec<u8> {
        &self.bytes
    }

    #[inline]
    pub(crate) fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }

    #[inline]
    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

impl fmt::Display for BString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_bstr(), f)
    }
}

impl fmt::Debug for BString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_bstr(), f)
    }
}

impl FromStr for BString {
    type Err = Utf8Error;

    #[inline]
    fn from_str(s: &str) -> Result<BString, Utf8Error> {
        Ok(BString::from(s))
    }
}

impl ops::Deref for BString {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Vec<u8> {
        self.as_vec()
    }
}

impl ops::DerefMut for BString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        self.as_vec_mut()
    }
}

impl AsRef<[u8]> for BString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<BStr> for BString {
    #[inline]
    fn as_ref(&self) -> &BStr {
        self.as_bstr()
    }
}

impl AsMut<[u8]> for BString {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl AsMut<BStr> for BString {
    #[inline]
    fn as_mut(&mut self) -> &mut BStr {
        self.as_mut_bstr()
    }
}

impl Borrow<[u8]> for BString {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<BStr> for BString {
    #[inline]
    fn borrow(&self) -> &BStr {
        self.as_bstr()
    }
}

impl Borrow<BStr> for Vec<u8> {
    #[inline]
    fn borrow(&self) -> &BStr {
        self.as_slice().as_bstr()
    }
}

impl Borrow<BStr> for String {
    #[inline]
    fn borrow(&self) -> &BStr {
        self.as_bytes().as_bstr()
    }
}

impl BorrowMut<[u8]> for BString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl BorrowMut<BStr> for BString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut BStr {
        self.as_mut_bstr()
    }
}

impl BorrowMut<BStr> for Vec<u8> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut BStr {
        BStr::new_mut(self.as_mut_slice())
    }
}

impl ToOwned for BStr {
    type Owned = BString;

    #[inline]
    fn to_owned(&self) -> BString {
        BString::from(self)
    }
}

impl Default for BString {
    fn default() -> BString {
        BString::from(vec![])
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for BString {
    #[inline]
    fn from(s: &'a [u8; N]) -> BString {
        BString::from(&s[..])
    }
}

impl<const N: usize> From<[u8; N]> for BString {
    #[inline]
    fn from(s: [u8; N]) -> BString {
        BString::from(&s[..])
    }
}

impl<'a> From<&'a [u8]> for BString {
    #[inline]
    fn from(s: &'a [u8]) -> BString {
        BString::from(s.to_vec())
    }
}

impl From<Vec<u8>> for BString {
    #[inline]
    fn from(s: Vec<u8>) -> BString {
        BString::new(s)
    }
}

impl From<BString> for Vec<u8> {
    #[inline]
    fn from(s: BString) -> Vec<u8> {
        s.into_vec()
    }
}

impl<'a> From<&'a str> for BString {
    #[inline]
    fn from(s: &'a str) -> BString {
        BString::from(s.as_bytes().to_vec())
    }
}

impl From<String> for BString {
    #[inline]
    fn from(s: String) -> BString {
        BString::from(s.into_bytes())
    }
}

impl<'a> From<&'a BStr> for BString {
    #[inline]
    fn from(s: &'a BStr) -> BString {
        BString::from(s.bytes.to_vec())
    }
}

impl<'a> From<BString> for Cow<'a, BStr> {
    #[inline]
    fn from(s: BString) -> Cow<'a, BStr> {
        Cow::Owned(s)
    }
}

impl TryFrom<BString> for String {
    type Error = FromUtf8Error;

    #[inline]
    fn try_from(s: BString) -> Result<String, FromUtf8Error> {
        s.into_vec().into_string()
    }
}

impl<'a> TryFrom<&'a BString> for &'a str {
    type Error = Utf8Error;

    #[inline]
    fn try_from(s: &'a BString) -> Result<&'a str, Utf8Error> {
        s.as_bytes().to_str()
    }
}

impl FromIterator<char> for BString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> BString {
        BString::from(iter.into_iter().collect::<String>())
    }
}

impl FromIterator<u8> for BString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> BString {
        BString::from(iter.into_iter().collect::<Vec<u8>>())
    }
}

impl<'a> FromIterator<&'a str> for BString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> BString {
        let mut buf = vec![];
        for b in iter {
            buf.push_str(b);
        }
        BString::from(buf)
    }
}

impl<'a> FromIterator<&'a [u8]> for BString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a [u8]>>(iter: T) -> BString {
        let mut buf = vec![];
        for b in iter {
            buf.push_str(b);
        }
        BString::from(buf)
    }
}

impl<'a> FromIterator<&'a BStr> for BString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a BStr>>(iter: T) -> BString {
        let mut buf = vec![];
        for b in iter {
            buf.push_str(b);
        }
        BString::from(buf)
    }
}

impl FromIterator<BString> for BString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = BString>>(iter: T) -> BString {
        let mut buf = vec![];
        for b in iter {
            buf.push_str(b);
        }
        BString::from(buf)
    }
}

impl Eq for BString {}

impl PartialEq for BString {
    #[inline]
    fn eq(&self, other: &BString) -> bool {
        &self[..] == &other[..]
    }
}

impl_partial_eq!(BString, Vec<u8>);
impl_partial_eq!(BString, [u8]);
impl_partial_eq!(BString, &'a [u8]);
impl_partial_eq!(BString, String);
impl_partial_eq!(BString, str);
impl_partial_eq!(BString, &'a str);
impl_partial_eq!(BString, BStr);
impl_partial_eq!(BString, &'a BStr);

impl PartialOrd for BString {
    #[inline]
    fn partial_cmp(&self, other: &BString) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.as_bytes(), other.as_bytes())
    }
}

impl Ord for BString {
    #[inline]
    fn cmp(&self, other: &BString) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl_partial_ord!(BString, Vec<u8>);
impl_partial_ord!(BString, [u8]);
impl_partial_ord!(BString, &'a [u8]);
impl_partial_ord!(BString, String);
impl_partial_ord!(BString, str);
impl_partial_ord!(BString, &'a str);
impl_partial_ord!(BString, BStr);
impl_partial_ord!(BString, &'a BStr);

#[macro_use]
mod macros;

mod ascii;
mod bstr;
mod bstring;
mod byteset;
mod error;
mod escape_bytes;
mod ext_slice;
mod ext_vec;
mod format;
mod helper;
mod hexdigit;
mod split;
mod unicode;
mod utf8;

pub use self::ascii::first_non_ascii_byte;
pub use self::bstr::BStr;
pub use self::bstring::BString;
pub use self::byteset::*;
pub use self::error::{FromUtf8Error, Utf8Error};
pub use self::escape_bytes::EscapeBytes;
pub use self::ext_slice::ByteSlice;
pub use self::ext_vec::{concat, join, ByteVec, DrainBytes};
pub use self::format::*;
pub use self::helper::*;
pub use self::hexdigit::*;
pub use self::split::*;
pub use self::unicode::*;
pub use self::utf8::{
    decode as decode_utf8, decode_last as decode_last_utf8, CharIndices,
    Chars, Utf8Chunk, Utf8Chunks,
};

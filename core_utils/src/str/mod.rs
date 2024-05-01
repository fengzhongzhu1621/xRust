#[macro_use]
mod macros;

mod ascii;
mod byteset;
mod ext_slice;
mod split;
mod unicode;
mod utf8;
mod bstr;
mod bstring;
mod error;
mod ext_vec;
mod escape_bytes;

pub use bstr::BStr;
pub use ascii::first_non_ascii_byte;
pub use byteset::*;
pub use ext_slice::ByteSlice;
pub use split::*;
pub use unicode::*;
pub use utf8::{
    decode as decode_utf8, decode_last as decode_last_utf8, CharIndices,
    Chars, Utf8Chunk, Utf8Chunks,
};
pub use bstring::BString;
pub use error::{FromUtf8Error, Utf8Error};
pub use ext_vec::{concat, join, ByteVec, DrainBytes};
pub use escape_bytes::EscapeBytes;

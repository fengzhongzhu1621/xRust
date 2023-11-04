#![cfg(windows)]
#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

mod types;
mod sys;
mod buffer;
mod clipboard;
mod raw;

pub use error_code::ErrorCode;
///Alias to result used by this crate
pub type SysResult<T> = Result<T, ErrorCode>;

pub use clipboard::Clipboard;

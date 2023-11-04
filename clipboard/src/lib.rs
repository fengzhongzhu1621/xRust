#![cfg(windows)]
#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

mod clipboard;
mod data;

pub use error_code::ErrorCode;
pub use clipboard::Clipboard;

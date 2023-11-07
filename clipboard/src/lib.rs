#![cfg(windows)]
#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

extern crate alloc;

mod clipboard;
mod action;

pub use error_code::ErrorCode;
pub use clipboard::Clipboard;

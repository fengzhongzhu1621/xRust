#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[macro_use]
pub mod macros;
#[macro_use]
pub mod condition;

pub mod aes;
pub mod cached_bool;
pub mod console;
pub mod convert;
pub mod datetime;
pub mod error;
pub mod ffi;
pub mod file;
pub mod hash;
pub mod image;
pub mod kv;
pub mod logger;
pub mod maybe_static;
pub mod path;
pub mod random;
pub mod str;
pub mod time;
pub mod var;
pub mod vec2;
pub mod zip;

mod set;

pub use aes::*;
pub use random::*;
pub use zip::*;

extern crate alloc;

#[cfg(feature = "serialize")]
extern crate serde;
#[macro_use]
#[cfg(feature = "serialize")]
extern crate serde_derive;

#[cfg(feature = "serialize")]
extern crate stfu8;

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[macro_use]
pub mod macros;
#[macro_use]
pub mod condition;

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
pub mod maybe_static;
pub mod path;
pub mod random;
pub mod time;
pub mod var;
pub mod vec2;

mod set;

pub use random::*;

extern crate alloc;

#[cfg(feature = "serialize")]
extern crate serde;
#[macro_use]
#[cfg(feature = "serialize")]
extern crate serde_derive;

#[cfg(feature = "serialize")]
extern crate stfu8;

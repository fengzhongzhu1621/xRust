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
pub mod displayable;
pub mod error;
pub mod file;
pub mod hash;
pub mod image;
pub mod kv;
pub mod logger;
pub mod matcher;
pub mod maybe_static;
pub mod panic;
pub mod path;
pub mod platform;
pub mod predicates;
pub mod random;
mod set;
pub mod signal;
pub mod str;
pub mod time;
pub mod var;
pub mod vec2;
pub mod version;
pub mod zip;

extern crate alloc;

#[cfg(feature = "serialize")]
extern crate serde;
#[macro_use]
#[cfg(feature = "serialize")]
extern crate serde_derive;

#[cfg(feature = "serialize")]
extern crate stfu8;

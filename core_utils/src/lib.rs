#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[macro_use]
pub mod macros;
#[macro_use]
pub mod condition;

pub mod aes;
pub mod cached_bool;
pub mod cmd;
pub mod console;
pub mod convert;
pub mod datetime;
pub mod debug;
pub mod displayable;
pub mod error;
pub mod file;
pub mod hash;
pub mod image;
pub mod iterator;
pub mod kv;
pub mod logger;
pub mod matcher;
pub mod maybe_static;
pub mod panic;
pub mod path;
pub mod platform;
pub mod predicates;
pub mod process;
pub mod random;
pub mod regex;
pub mod signal;
pub mod str;
pub mod time;
pub mod var;
pub mod vec2;
pub mod version;
pub mod zip;

mod set;

extern crate alloc;

#[cfg(feature = "serialize")]
extern crate serde;
#[macro_use]
#[cfg(feature = "serialize")]
extern crate serde_derive;

#[cfg(feature = "serialize")]
extern crate stfu8;

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
pub mod kv;
pub mod maybe_static;
pub mod path;
pub mod time;
pub mod var;
pub mod vec2;

mod set;

extern crate alloc;

#[macro_use]
pub mod condition;

pub mod convert;
pub mod kv;
pub mod maybe_static;
// pub use convert::*;
//pub use maybe_static::*;
pub mod cached_bool;
pub mod console;
pub mod error;
pub mod ffi;
pub mod time;
pub mod var;

extern crate alloc;

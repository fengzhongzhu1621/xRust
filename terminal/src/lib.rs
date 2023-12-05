#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "_nightly", feature(doc_cfg))]
//#![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[macro_use]
mod macros;

pub mod attr;
pub mod color;
pub mod quirk;
pub mod style;
pub mod terminal;

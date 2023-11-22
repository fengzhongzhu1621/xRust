#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "_nightly", feature(doc_cfg))]
//#![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

pub mod terminal;

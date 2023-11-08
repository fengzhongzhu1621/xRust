#[macro_use]
mod messages;
mod builder;
mod env;
mod error;
pub mod filter;
pub mod fmt;
mod level;
pub mod logger;
mod meta_data;
mod record;
mod var;

pub use builder::Builder;
pub use env::{Env, Var};
pub use error::{ParseLevelError, SetLoggerError};
pub use level::{Level, LevelFilter, STATIC_MAX_LEVEL};
pub use logger::*;
pub use messages::*;
pub use meta_data::{Metadata, MetadataBuilder};
pub use record::{Record, RecordBuilder};
pub use var::Var;

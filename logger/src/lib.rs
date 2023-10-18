#[macro_use]
mod messages;
mod error;
mod level;
mod meta_data;
mod record;

pub mod logger;

pub use error::{ParseLevelError, SetLoggerError};
pub use level::{Level, LevelFilter, STATIC_MAX_LEVEL};
pub use logger::*;
pub use messages::*;
pub use meta_data::{Metadata, MetadataBuilder};
pub use record::{Record, RecordBuilder};

#[macro_use]
mod messages;
mod error;
mod level;
mod meta_data;
mod record;
mod std_logger;

pub mod logger;

pub use error::{ParseLevelError, SetLoggerError};
pub use level::{Level, LevelFilter, STATIC_MAX_LEVEL};
pub use logger::*;
pub use messages::*;
pub use meta_data::*;
pub use record::{Record, RecordBuilder};
pub use std_logger::*;

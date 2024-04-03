#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub use self::unix as sys;

#[cfg(windows)]
pub use self::windows as sys;

pub mod buffer;
pub mod fmt;
pub mod string;
pub mod types;

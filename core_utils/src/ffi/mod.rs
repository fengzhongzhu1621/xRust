pub mod buffer;
pub mod types;

#[cfg(unix)]
pub mod lib_c;
#[cfg(unix)]
pub use lib_c as sys;

#[cfg(windows)]
pub mod win;
#[cfg(windows)]
pub use win as sys;

pub mod string;

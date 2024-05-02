#![cfg(windows)]
pub mod clipboard;
pub mod constants;
pub mod dc;
pub mod error;
pub mod file;
pub mod lock;
pub mod mem;
pub mod os_info;
pub mod process;
pub mod process;
pub mod signal;
pub mod system;
pub mod types;
pub mod utils;

pub use error::{Error, SignalError};
pub use file::{
    close, fsync, lock, open, pid, truncate, try_lock, unlock, write, OsStr,
    OsString,
};
pub use process::wait_timeout;
pub use signal::{block_ctrl_c, init_os_handler};
pub use types::{FileDesc, Signal};

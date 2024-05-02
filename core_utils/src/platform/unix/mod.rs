pub mod lib_c;
pub mod nix;

pub use self::nix::os_handler::{block_ctrl_c, init_os_handler};
pub use self::nix::signal::Signal;

pub use self::nix::error::SignalError;
pub use lib_c::error::Error;
pub use lib_c::file::{
    close, fsync, lock, open, pid, truncate, try_lock, unlock, write, FileDesc,
};
pub use lib_c::os_str::OsStr;
pub use lib_c::os_string::OsString;
pub use lib_c::process::wait_timeout;

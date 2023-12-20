mod abs;
mod error;
mod file_edit;
mod file_open;
mod file_read;
mod file_write;
mod path_dir;
mod path_file;
mod path_info;
mod path_mut;
mod path_ops;
mod path_type;
#[cfg(feature = "serialize")]
mod ser;

pub use abs::PathAbs;
pub use error::{Error, Result};
pub use file_edit::FileEdit;
pub use file_open::FileOpen;
pub use file_read::FileRead;
pub use file_write::FileWrite;
pub use path_dir::PathDir;
pub use path_file::PathFile;
pub use path_info::PathInfo;
pub use path_mut::PathMut;
pub use path_ops::PathOps;
pub use path_type::PathType;
#[cfg(feature = "serialize")]
pub use ser::{PathSer, ToStfu8};

use regex::Regex;
use std::path::Path;

pub fn escape<P: AsRef<Path>>(path: P) -> String {
    regex::escape(&format!("{}", path.as_ref().display()))
}

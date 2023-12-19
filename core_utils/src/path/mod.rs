mod abs;
mod error;
mod path_info;
mod path_mut;
mod path_ops;

pub use abs::PathAbs;
pub use error::{Error, Result};
pub use path_info::PathInfo;
pub use path_mut::PathMut;
pub use path_ops::PathOps;

use regex::Regex;
use std::path::Path;

pub fn escape<P: AsRef<Path>>(path: P) -> String {
    regex::escape(&format!("{}", path.as_ref().display()))
}

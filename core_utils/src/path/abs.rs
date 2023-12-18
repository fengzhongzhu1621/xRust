use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// An absolute (not _necessarily_ [canonicalized][1]) path that may or may not exist.
///
/// [1]: https://doc.rust-lang.org/std/path/struct.Path.html?search=#method.canonicalize
pub struct PathAbs(pub(crate) Arc<PathBuf>);

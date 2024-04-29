mod existence;
mod fc;
mod fs;
mod ft;

pub use existence::{exists, missing, ExistencePredicate};
pub use fc::{FileContentPredicate, PredicateFileContentExt};
pub use fs::{eq_file, BinaryFilePredicate, StrFilePredicate};
pub use ft::{is_dir, is_file, is_symlink, FileTypePredicate};

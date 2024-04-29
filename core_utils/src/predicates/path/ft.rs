use std::fmt;
use std::io;
use std::path;

use crate::file::FileType;
use crate::predicates::core::{
    Case, Palette, Parameter, Predicate, PredicateReflection, Product,
};

/// Predicate that checks the `std::fs::FileType`.
///
/// This is created by the `predicate::path::is_file`, `predicate::path::is_dir`, and `predicate::path::is_symlink`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTypePredicate {
    ft: FileType, // 文件类型
    follow: bool,
}

impl FileTypePredicate {
    /// Follow symbolic links.
    ///
    /// When yes is true, symbolic links are followed as if they were normal directories and files.
    ///
    /// Default: disabled.
    pub fn follow_links(mut self, yes: bool) -> Self {
        self.follow = yes;
        self
    }

    /// 构造函数
    /// Allow to create an `FileTypePredicate` from a `path`
    pub fn from_path(path: &path::Path) -> io::Result<FileTypePredicate> {
        Ok(FileTypePredicate {
            ft: FileType::from_path(path, true)?,
            follow: true,
        })
    }
}

impl Predicate<path::Path> for FileTypePredicate {
    fn eval(&self, path: &path::Path) -> bool {
        let metadata = if self.follow {
            path.metadata()
        } else {
            path.symlink_metadata()
        };
        // 判断文件类型是否相同
        metadata.map(|m| self.ft.eval(m.file_type())).unwrap_or(false)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &path::Path,
    ) -> Option<Case<'a>> {
        let actual_type = FileType::from_path(variable, self.follow);
        match (expected, actual_type) {
            (_, Ok(actual_type)) => {
                // 判断文件类型是否相同
                let result = self.ft == actual_type;
                if result == expected {
                    Some(Case::new(Some(self), result).add_product(
                        Product::new("actual filetype", actual_type),
                    ))
                } else {
                    None
                }
            }
            (true, Err(_)) => None,
            (false, Err(err)) => Some(
                Case::new(Some(self), false)
                    .add_product(Product::new("error", err)),
            ),
        }
    }
}

impl PredicateReflection for FileTypePredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("follow", &self.follow)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for FileTypePredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),
            palette.description("is"),
            palette.expected(self.ft)
        )
    }
}

/// Creates a new `Predicate` that ensures the path points to a file.
pub fn is_file() -> FileTypePredicate {
    FileTypePredicate { ft: FileType::File, follow: false }
}

/// Creates a new `Predicate` that ensures the path points to a directory.
pub fn is_dir() -> FileTypePredicate {
    FileTypePredicate { ft: FileType::Dir, follow: false }
}

/// Creates a new `Predicate` that ensures the path points to a symlink.
pub fn is_symlink() -> FileTypePredicate {
    FileTypePredicate { ft: FileType::Symlink, follow: false }
}

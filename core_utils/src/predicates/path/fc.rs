use crate::path::read_file;
use crate::predicates::core::{
    default_find_case, Case, Child, Palette, Parameter, Predicate,
    PredicateReflection, Product,
};
use std::fmt;
use std::path;

/// 定义一个文件内容断言
/// Predicate adapter that converts a `path` predicate to a byte predicate on its content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    p: P,
}

impl<P> PredicateReflection for FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params = vec![Child::new("predicate", &self.p)];
        Box::new(params.into_iter())
    }
}

impl<P> fmt::Display for FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.p.fmt(f)
    }
}

impl<P> Predicate<path::Path> for FileContentPredicate<P>
where
    P: Predicate<[u8]>,
{
    fn eval(&self, path: &path::Path) -> bool {
        self.eval(path).unwrap_or(false)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &path::Path,
    ) -> Option<Case<'a>> {
        let buffer = read_file(variable);
        match (expected, buffer) {
            (_, Ok(buffer)) => {
                // 读文件成功，且断言成功
                self.p.find_case(expected, &buffer).map(|case| {
                    case.add_product(Product::new(
                        "var",
                        variable.display().to_string(),
                    ))
                })
            }
            (true, Err(_)) => None,
            (false, Err(err)) => Some(
                // 读文件失败，且断言失败
                Case::new(Some(self), false)
                    .add_product(Product::new(
                        "var",
                        variable.display().to_string(),
                    ))
                    .add_product(Product::new("error", err)),
            ),
        }
    }
}

/// `Predicate` extension adapting a `slice` Predicate.
pub trait PredicateFileContentExt
where
    Self: Predicate<[u8]>,
    Self: Sized,
{
    // 添加扩展功能，创建一个新的断言
    #[allow(clippy::wrong_self_convention)]
    fn from_file_path(self) -> FileContentPredicate<Self> {
        FileContentPredicate { p: self }
    }
}

/// 给 Predicate 添加扩展功能
impl<P> PredicateFileContentExt for P where P: Predicate<[u8]> {}

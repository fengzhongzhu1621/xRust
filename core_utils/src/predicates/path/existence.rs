use crate::predicates::core::{
    default_find_case, Case, Palette, Parameter, Predicate,
    PredicateReflection, Product,
};

use std::{fmt, path};

// 定义一个路径是否存在的断言
/// Predicate that checks if a file is present
///
/// This is created by the `predicate::path::exists` and `predicate::path::missing`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExistencePredicate {
    exists: bool,
}

/// 路径是否存在的断言
impl Predicate<path::Path> for ExistencePredicate {
    // 判断路径是否存在
    fn eval(&self, path: &path::Path) -> bool {
        path.exists() == self.exists
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &path::Path,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable).map(|case| {
            case.add_product(Product::new(
                "var",
                variable.display().to_string(),
            ))
        })
    }
}

impl PredicateReflection for ExistencePredicate {}

impl fmt::Display for ExistencePredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{:#}({:#})",
            palette.description(if self.exists {
                "exists"
            } else {
                "missing"
            }),
            palette.var("var")
        )
    }
}

/// 定义一个路径存在断言
pub fn exists() -> ExistencePredicate {
    ExistencePredicate { exists: true }
}

/// 定义一个路径不存在断言
pub fn missing() -> ExistencePredicate {
    ExistencePredicate { exists: false }
}

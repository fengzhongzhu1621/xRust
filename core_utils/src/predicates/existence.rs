use super::case::Case;
use super::predicate::Predicate;
use super::product::Product;

/// Predicate that checks if a file is present
///
/// This is created by the `predicate::path::exists` and `predicate::path::missing`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExistencePredicate {
    exists: bool,
}

impl Predicate<path::Path> for ExistencePredicate {
    fn eval(&self, path: &path::Path) -> bool {
        path.exists() == self.exists
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &path::Path,
    ) -> Option<Case<'a>> {
        utils::default_find_case(self, expected, variable).map(|case| {
            case.add_product(Product::new(
                "var",
                variable.display().to_string(),
            ))
        })
    }
}

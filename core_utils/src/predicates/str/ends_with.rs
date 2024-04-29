use crate::predicates::core::{
    default_find_case, Case, Palette, Predicate, PredicateReflection, Product,
};
use std::fmt;

/// Predicate checks end of str
///
/// This is created by `predicates::str::ends_with`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndsWithPredicate {
    pattern: String,
}

impl Predicate<str> for EndsWithPredicate {
    fn eval(&self, variable: &str) -> bool {
        variable.ends_with(&self.pattern)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &str,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable).map(|case| {
            case.add_product(Product::new("var", variable.to_owned()))
        })
    }
}

impl PredicateReflection for EndsWithPredicate {}

impl fmt::Display for EndsWithPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{}.{}({:?})",
            palette.var("var"),
            palette.description("ends_with"),
            self.pattern
        )
    }
}
/// Creates a new `Predicate` that ensures a str ends with `pattern`
pub fn ends_with<P>(pattern: P) -> EndsWithPredicate
where
    P: Into<String>,
{
    EndsWithPredicate { pattern: pattern.into() }
}

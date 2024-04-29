use crate::predicates::core::{
    default_find_case, Case, Palette, Predicate, PredicateReflection, Product,
};
use std::fmt;

/// Predicate checks start of str
///
/// This is created by `predicates::str::starts_with`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartsWithPredicate {
    pattern: String,
}

impl Predicate<str> for StartsWithPredicate {
    fn eval(&self, variable: &str) -> bool {
        variable.starts_with(&self.pattern)
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

impl PredicateReflection for StartsWithPredicate {}

impl fmt::Display for StartsWithPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{}.{}({:?})",
            palette.var("var"),
            palette.description("starts_with"),
            self.pattern
        )
    }
}

/// Creates a new `Predicate` that ensures a str starts with `pattern`
pub fn starts_with<P>(pattern: P) -> StartsWithPredicate
where
    P: Into<String>,
{
    StartsWithPredicate { pattern: pattern.into() }
}

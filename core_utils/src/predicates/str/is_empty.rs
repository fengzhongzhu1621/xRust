use crate::predicates::core::{
    default_find_case, Case, Palette, Predicate, PredicateReflection, Product,
};
use std::fmt;

/// Predicate that checks for empty strings.
///
/// This is created by `predicates::str::is_empty`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsEmptyPredicate {}

impl Predicate<str> for IsEmptyPredicate {
    fn eval(&self, variable: &str) -> bool {
        variable.is_empty()
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

impl PredicateReflection for IsEmptyPredicate {}

impl fmt::Display for IsEmptyPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{:#}.{:#}()",
            palette.var("var"),
            palette.description("is_empty"),
        )
    }
}

pub fn is_empty() -> IsEmptyPredicate {
    IsEmptyPredicate {}
}

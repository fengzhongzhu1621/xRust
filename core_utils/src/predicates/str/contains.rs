use super::matches::MatchesPredicate;
use crate::predicates::core::{
    default_find_case, Case, Palette, Predicate, PredicateReflection, Product,
};
use std::fmt;

/// Predicate that checks for patterns.
///
/// This is created by `predicates::str:contains`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainsPredicate {
    pattern: String,
}

impl ContainsPredicate {
    /// Require a specific count of matches.
    pub fn count(self, count: usize) -> MatchesPredicate {
        MatchesPredicate { pattern: self.pattern, count }
    }
}

impl Predicate<str> for ContainsPredicate {
    fn eval(&self, variable: &str) -> bool {
        variable.contains(&self.pattern)
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

impl PredicateReflection for ContainsPredicate {}

impl fmt::Display for ContainsPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{}.{}({})",
            palette.var("var"),
            palette.description("contains"),
            palette.expected(&self.pattern),
        )
    }
}

/// Creates a new `Predicate` that ensures a str contains `pattern`
pub fn contains<P>(pattern: P) -> ContainsPredicate
where
    P: Into<String>,
{
    ContainsPredicate { pattern: pattern.into() }
}

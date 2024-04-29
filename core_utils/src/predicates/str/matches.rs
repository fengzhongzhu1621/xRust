use crate::predicates::core::{
    Case, Palette, Parameter, Predicate, PredicateReflection, Product,
};
use std::fmt;

/// Predicate that checks for repeated patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchesPredicate {
    pub pattern: String,
    pub count: usize,
}

impl Predicate<str> for MatchesPredicate {
    fn eval(&self, variable: &str) -> bool {
        variable.matches(&self.pattern).count() == self.count
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &str,
    ) -> Option<Case<'a>> {
        let actual_count = variable.matches(&self.pattern).count();
        let result = self.count == actual_count;
        if result == expected {
            Some(
                Case::new(Some(self), result)
                    .add_product(Product::new("var", variable.to_owned()))
                    .add_product(Product::new("actual count", actual_count)),
            )
        } else {
            None
        }
    }
}

impl PredicateReflection for MatchesPredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("count", &self.count)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for MatchesPredicate {
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

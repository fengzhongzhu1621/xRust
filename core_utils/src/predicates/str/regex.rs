use crate::predicates::core::{
    default_find_case, Case, Palette, Parameter, Predicate,
    PredicateReflection, Product,
};
use regex;
use std::fmt;

/// An error that occurred during parsing or compiling a regular expression.
pub type RegexError = regex::Error;

#[derive(Debug, Clone)]
pub struct RegexMatchesPredicate {
    re: regex::Regex,
    count: usize,
}

impl Predicate<str> for RegexMatchesPredicate {
    fn eval(&self, variable: &str) -> bool {
        self.re.find_iter(variable).count() == self.count
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &str,
    ) -> Option<Case<'a>> {
        let actual_count = self.re.find_iter(variable).count();
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

impl PredicateReflection for RegexMatchesPredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("count", &self.count)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for RegexMatchesPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{}.{}({})",
            palette.var("var"),
            palette.description("is_match"),
            palette.expected(&self.re),
        )
    }
}

/// Predicate that uses regex matching
///
/// This is created by the `predicate::str::is_match`.
#[derive(Debug, Clone)]
pub struct RegexPredicate {
    re: regex::Regex,
}

impl RegexPredicate {
    /// Require a specific count of matches.
    pub fn count(self, count: usize) -> RegexMatchesPredicate {
        RegexMatchesPredicate { re: self.re, count }
    }
}

impl Predicate<str> for RegexPredicate {
    fn eval(&self, variable: &str) -> bool {
        self.re.is_match(variable)
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

impl PredicateReflection for RegexPredicate {}

impl fmt::Display for RegexPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{}.{}({})",
            palette.var("var"),
            palette.description("is_match"),
            palette.expected(&self.re),
        )
    }
}

pub fn is_match<S>(pattern: S) -> Result<RegexPredicate, RegexError>
where
    S: AsRef<str>,
{
    regex::Regex::new(pattern.as_ref()).map(|re| RegexPredicate { re })
}

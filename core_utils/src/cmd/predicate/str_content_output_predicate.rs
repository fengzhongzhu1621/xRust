use predicates;
use predicates::str::PredicateStrExt;
use predicates_core;
use std::fmt;

#[derive(Debug, Clone)]
pub struct StrContentOutputPredicate(
    predicates::str::Utf8Predicate<predicates::str::DifferencePredicate>,
);

impl StrContentOutputPredicate {
    pub(crate) fn from_str(value: &'static str) -> Self {
        let pred = predicates::str::diff(value).from_utf8();
        StrContentOutputPredicate(pred)
    }

    pub(crate) fn from_string(value: String) -> Self {
        let pred = predicates::str::diff(value).from_utf8();
        StrContentOutputPredicate(pred)
    }
}

impl predicates_core::reflection::PredicateReflection
    for StrContentOutputPredicate
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<
        dyn Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a,
    > {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Child<'a>> + 'a>
    {
        self.0.children()
    }
}

impl predicates_core::Predicate<[u8]> for StrContentOutputPredicate {
    fn eval(&self, item: &[u8]) -> bool {
        self.0.eval(item)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &[u8],
    ) -> Option<predicates_core::reflection::Case<'a>> {
        self.0.find_case(expected, variable)
    }
}

impl fmt::Display for StrContentOutputPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

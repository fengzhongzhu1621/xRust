use predicates;
use predicates_core;
use std::fmt;

#[derive(Debug)]
pub struct InCodePredicate(predicates::iter::InPredicate<i32>);

impl InCodePredicate {
    pub(crate) fn new<I: IntoIterator<Item = i32>>(value: I) -> Self {
        let pred = predicates::iter::in_iter(value);
        InCodePredicate(pred)
    }
}

impl predicates_core::reflection::PredicateReflection for InCodePredicate {
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

impl predicates_core::Predicate<i32> for InCodePredicate {
    fn eval(&self, item: &i32) -> bool {
        self.0.eval(item)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &i32,
    ) -> Option<predicates_core::reflection::Case<'a>> {
        self.0.find_case(expected, variable)
    }
}

impl fmt::Display for InCodePredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

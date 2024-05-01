use predicates;
use predicates::str::PredicateStrExt;
use predicates_core;
use std::fmt;

#[derive(Debug, Clone)]
pub struct StrOutputPredicate<P: predicates_core::Predicate<str>>(
    predicates::str::Utf8Predicate<P>,
);

impl<P> StrOutputPredicate<P>
where
    P: predicates_core::Predicate<str>,
{
    pub(crate) fn new(pred: P) -> Self {
        let pred = pred.from_utf8();
        StrOutputPredicate(pred)
    }
}

impl<P> predicates_core::reflection::PredicateReflection
    for StrOutputPredicate<P>
where
    P: predicates_core::Predicate<str>,
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

impl<P> predicates_core::Predicate<[u8]> for StrOutputPredicate<P>
where
    P: predicates_core::Predicate<str>,
{
    fn eval(&self, item: &[u8]) -> bool {
        // impl<P> Predicate<[u8]> for Utf8Predicate<P>
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

impl<P> fmt::Display for StrOutputPredicate<P>
where
    P: predicates_core::Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

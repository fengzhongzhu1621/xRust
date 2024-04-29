use super::NormalizedPredicate;
use crate::predicates::core::Predicate;

/// `Predicate` extension adapting a `str` Predicate.
pub trait PredicateStrExt
where
    Self: Predicate<str>,
    Self: Sized,
{
    /// Returns a `NormalizedPredicate` that ensures
    fn normalize(self) -> NormalizedPredicate<Self> {
        NormalizedPredicate { p: self }
    }
}

impl<P> PredicateStrExt for P where P: Predicate<str> {}

use super::{NormalizedPredicate, TrimPredicate, Utf8Predicate};
use crate::predicates::core::Predicate;

/// `Predicate` extension adapting a `str` Predicate.
pub trait PredicateStrExt
where
    Self: Predicate<str>,
    Self: Sized,
{
    /// Returns a `TrimPredicate` that ensures the data passed to `Self` is trimmed.
    fn trim(self) -> TrimPredicate<Self> {
        TrimPredicate { p: self }
    }

    /// Returns a `Utf8Predicate` that adapts `Self` to a `[u8]` `Predicate`.
    #[allow(clippy::wrong_self_convention)]
    fn from_utf8(self) -> Utf8Predicate<Self> {
        Utf8Predicate { p: self }
    }

    /// Returns a `NormalizedPredicate` that ensures
    fn normalize(self) -> NormalizedPredicate<Self> {
        NormalizedPredicate { p: self }
    }
}

impl<P> PredicateStrExt for P where P: Predicate<str> {}

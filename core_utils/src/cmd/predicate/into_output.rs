use super::{
    BytesContentOutputPredicate, StrContentOutputPredicate, StrOutputPredicate,
};
use predicates_core;

pub trait IntoOutputPredicate<P>
where
    P: predicates_core::Predicate<[u8]>,
{
    /// The type of the predicate being returned.
    type Predicate;

    /// Convert to a predicate for testing a path.
    fn into_output(self) -> P;
}

impl<P> IntoOutputPredicate<P> for P
where
    P: predicates_core::Predicate<[u8]>,
{
    type Predicate = P;

    fn into_output(self) -> Self::Predicate {
        self
    }
}

impl<P> IntoOutputPredicate<StrOutputPredicate<P>> for P
where
    P: predicates_core::Predicate<str>,
{
    type Predicate = StrOutputPredicate<P>;

    fn into_output(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

impl IntoOutputPredicate<BytesContentOutputPredicate> for Vec<u8> {
    type Predicate = BytesContentOutputPredicate;

    fn into_output(self) -> Self::Predicate {
        Self::Predicate::from_vec(self)
    }
}

impl IntoOutputPredicate<BytesContentOutputPredicate> for &'static [u8] {
    type Predicate = BytesContentOutputPredicate;

    fn into_output(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

impl IntoOutputPredicate<StrContentOutputPredicate> for String {
    type Predicate = StrContentOutputPredicate;

    fn into_output(self) -> Self::Predicate {
        Self::Predicate::from_string(self)
    }
}

impl IntoOutputPredicate<StrContentOutputPredicate> for &'static str {
    type Predicate = StrContentOutputPredicate;

    fn into_output(self) -> Self::Predicate {
        Self::Predicate::from_str(self)
    }
}

use predicates;
use predicates_core;
use std::borrow::Cow;
use std::fmt;

#[derive(Debug)]
pub struct BytesContentOutputPredicate(Cow<'static, [u8]>);

impl BytesContentOutputPredicate {
    pub(crate) fn new(value: &'static [u8]) -> Self {
        BytesContentOutputPredicate(Cow::from(value))
    }

    pub(crate) fn from_vec(value: Vec<u8>) -> Self {
        BytesContentOutputPredicate(Cow::from(value))
    }
}

impl predicates_core::reflection::PredicateReflection
    for BytesContentOutputPredicate
{
}

impl predicates_core::Predicate<[u8]> for BytesContentOutputPredicate {
    fn eval(&self, item: &[u8]) -> bool {
        self.0.as_ref() == item
    }

    fn find_case(
        &self,
        expected: bool,
        variable: &[u8],
    ) -> Option<predicates_core::reflection::Case<'_>> {
        let actual = self.eval(variable);
        if expected == actual {
            Some(predicates_core::reflection::Case::new(Some(self), actual))
        } else {
            None
        }
    }
}

impl fmt::Display for BytesContentOutputPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        predicates::ord::eq(self.0.as_ref()).fmt(f)
    }
}

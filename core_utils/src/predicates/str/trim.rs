use crate::predicates::core::{Case, Child, Predicate, PredicateReflection};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimPredicate<P>
where
    P: Predicate<str>,
{
    pub(crate) p: P,
}

impl<P> Predicate<str> for TrimPredicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &str) -> bool {
        self.p.eval(variable.trim())
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &str,
    ) -> Option<Case<'a>> {
        self.p.find_case(expected, variable.trim())
    }
}

impl<P> PredicateReflection for TrimPredicate<P>
where
    P: Predicate<str>,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params = vec![Child::new("predicate", &self.p)];
        Box::new(params.into_iter())
    }
}

impl<P> fmt::Display for TrimPredicate<P>
where
    P: Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.p.fmt(f)
    }
}

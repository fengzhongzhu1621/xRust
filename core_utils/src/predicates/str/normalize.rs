use crate::predicates::core::{
    Case, Child, Palette, Parameter, Predicate, PredicateReflection, Product,
};
use std::fmt;

use crate::iterator::normalized;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalizedPredicate<P>
where
    P: Predicate<str>,
{
    pub(crate) p: P,
}

impl<P> PredicateReflection for NormalizedPredicate<P>
where
    P: Predicate<str>,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params = vec![Child::new("predicate", &self.p)];
        Box::new(params.into_iter())
    }
}

impl<P> Predicate<str> for NormalizedPredicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &str) -> bool {
        let variable = normalized(variable.chars()).collect::<String>();
        self.p.eval(&variable)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &str,
    ) -> Option<Case<'a>> {
        let variable = normalized(variable.chars()).collect::<String>();
        self.p.find_case(expected, &variable)
    }
}

impl<P> fmt::Display for NormalizedPredicate<P>
where
    P: Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.p.fmt(f)
    }
}

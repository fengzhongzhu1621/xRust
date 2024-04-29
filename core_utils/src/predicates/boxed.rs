use crate::predicates::core::{
    default_find_case, Case, Child, Parameter, Predicate,
    PredicateReflection,
};
use std::fmt;

/// `Predicate` that wraps another `Predicate` as a trait object, allowing
/// sized storage of predicate types.
pub struct BoxPredicate<Item: ?Sized>(Box<dyn Predicate<Item> + Send + Sync>);

impl<Item> BoxPredicate<Item>
where
    Item: ?Sized,
{
    /// Creates a new `BoxPredicate`, a wrapper around a dynamically-dispatched
    /// `Predicate` type with useful trait impls.
    pub fn new<P: Predicate<Item>>(inner: P) -> BoxPredicate<Item>
    where
        P: Send + Sync + 'static,
    {
        BoxPredicate(Box::new(inner))
    }
}

impl<Item> fmt::Debug for BoxPredicate<Item>
where
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxPredicate").finish()
    }
}

impl<Item> PredicateReflection for BoxPredicate<Item>
where
    Item: ?Sized,
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        self.0.children()
    }
}

impl<Item> fmt::Display for BoxPredicate<Item>
where
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<Item> Predicate<Item> for BoxPredicate<Item>
where
    Item: ?Sized,
{
    fn eval(&self, variable: &Item) -> bool {
        self.0.eval(variable)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Item,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable)
    }
}

/// `Predicate` extension for boxing a `Predicate`.
pub trait PredicateBoxExt<Item: ?Sized>
where
    Self: Predicate<Item>,
{
    fn boxed(self) -> BoxPredicate<Item>
    where
        Self: Sized + Send + Sync + 'static,
    {
        BoxPredicate::new(self)
    }
}

impl<P, Item> PredicateBoxExt<Item> for P where P: Predicate<Item> {}

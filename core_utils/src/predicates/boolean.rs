use crate::predicates::core::{
    default_find_case, Case, Child, Palette, Parameter, Predicate,
    PredicateReflection,
};
use std::fmt;
use std::marker::PhantomData;

/// Predicate that always returns a constant (boolean) result.
///
/// This is created by the `predicate::always` and `predicate::never` functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BooleanPredicate {
    retval: bool,
}

impl<Item: ?Sized> Predicate<Item> for BooleanPredicate {
    fn eval(&self, _variable: &Item) -> bool {
        self.retval
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Item,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable)
    }
}

impl PredicateReflection for BooleanPredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("value", &self.retval)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for BooleanPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(f, "{}", palette.expected(self.retval))
    }
}

pub fn always() -> BooleanPredicate {
    BooleanPredicate { retval: true }
}

pub fn never() -> BooleanPredicate {
    BooleanPredicate { retval: false }
}

/// Predicate that combines two `Predicate`s, returning the AND of the results.
///
/// This is created by the `Predicate::and` function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    a: M1,
    b: M2,
    _phantom: PhantomData<Item>,
}

unsafe impl<M1, M2, Item> Send for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item> + Send,
    M2: Predicate<Item> + Send,
    Item: ?Sized,
{
}

unsafe impl<M1, M2, Item> Sync for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item> + Sync,
    M2: Predicate<Item> + Sync,
    Item: ?Sized,
{
}

impl<M1, M2, Item> AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    /// Create a new `AndPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> AndPredicate<M1, M2, Item> {
        AndPredicate { a, b, _phantom: PhantomData }
    }
}

impl<M1, M2, Item> Predicate<Item> for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    fn eval(&self, item: &Item) -> bool {
        self.a.eval(item) && self.b.eval(item)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Item,
    ) -> Option<Case<'a>> {
        let child_a = self.a.find_case(expected, variable);
        match (expected, child_a) {
            (true, Some(child_a)) => {
                self.b.find_case(expected, variable).map(|child_b| {
                    Case::new(Some(self), expected)
                        .add_child(child_a)
                        .add_child(child_b)
                })
            }
            (true, None) => None,
            (false, Some(child_a)) => {
                Some(Case::new(Some(self), expected).add_child(child_a))
            }
            (false, None) => {
                self.b.find_case(expected, variable).map(|child_b| {
                    Case::new(Some(self), expected).add_child(child_b)
                })
            }
        }
    }
}

impl<M1, M2, Item> PredicateReflection for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params =
            vec![Child::new("left", &self.a), Child::new("right", &self.b)];
        Box::new(params.into_iter())
    }
}

impl<M1, M2, Item> fmt::Display for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} && {})", self.a, self.b)
    }
}

/// Predicate that returns a `Predicate` taking the logical NOT of the result.
///
/// This is created by the `Predicate::not` function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized,
{
    inner: M,
    _phantom: PhantomData<Item>,
}

unsafe impl<M, Item> Send for NotPredicate<M, Item>
where
    M: Predicate<Item> + Send,
    Item: ?Sized,
{
}

unsafe impl<M, Item> Sync for NotPredicate<M, Item>
where
    M: Predicate<Item> + Sync,
    Item: ?Sized,
{
}

impl<M, Item> NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized,
{
    /// Create a new `NotPredicate` over predicate `inner`.
    pub fn new(inner: M) -> NotPredicate<M, Item> {
        NotPredicate { inner, _phantom: PhantomData }
    }
}

impl<M, Item> Predicate<Item> for NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized,
{
    fn eval(&self, item: &Item) -> bool {
        !self.inner.eval(item)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Item,
    ) -> Option<Case<'a>> {
        self.inner
            .find_case(!expected, variable)
            .map(|child| Case::new(Some(self), expected).add_child(child))
    }
}

impl<M, Item> PredicateReflection for NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params = vec![Child::new("predicate", &self.inner)];
        Box::new(params.into_iter())
    }
}

impl<M, Item> fmt::Display for NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(! {})", self.inner)
    }
}

/// Predicate that combines two `Predicate`s, returning the OR of the results.
///
/// This is created by the `Predicate::or` function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    a: M1,
    b: M2,
    _phantom: PhantomData<Item>,
}

unsafe impl<M1, M2, Item> Send for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item> + Send,
    M2: Predicate<Item> + Send,
    Item: ?Sized,
{
}

unsafe impl<M1, M2, Item> Sync for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item> + Sync,
    M2: Predicate<Item> + Sync,
    Item: ?Sized,
{
}

impl<M1, M2, Item> OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    /// Create a new `OrPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> OrPredicate<M1, M2, Item> {
        OrPredicate { a, b, _phantom: PhantomData }
    }
}

impl<M1, M2, Item> Predicate<Item> for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    fn eval(&self, item: &Item) -> bool {
        self.a.eval(item) || self.b.eval(item)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Item,
    ) -> Option<Case<'a>> {
        let child_a = self.a.find_case(expected, variable);
        match (expected, child_a) {
            (true, Some(child_a)) => {
                Some(Case::new(Some(self), expected).add_child(child_a))
            }
            (true, None) => {
                self.b.find_case(expected, variable).map(|child_b| {
                    Case::new(Some(self), expected).add_child(child_b)
                })
            }
            (false, Some(child_a)) => {
                self.b.find_case(expected, variable).map(|child_b| {
                    Case::new(Some(self), expected)
                        .add_child(child_a)
                        .add_child(child_b)
                })
            }
            (false, None) => None,
        }
    }
}

impl<M1, M2, Item> PredicateReflection for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params =
            vec![Child::new("left", &self.a), Child::new("right", &self.b)];
        Box::new(params.into_iter())
    }
}

impl<M1, M2, Item> fmt::Display for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} || {})", self.a, self.b)
    }
}

/// `Predicate` extension that adds boolean logic.
pub trait PredicateBooleanExt<Item: ?Sized>
where
    Self: Predicate<Item>,
{
    /// Compute the logical AND of two `Predicate` results, returning the result.
    fn and<B>(self, other: B) -> AndPredicate<Self, B, Item>
    where
        B: Predicate<Item>,
        Self: Sized,
    {
        AndPredicate::new(self, other)
    }

    /// Compute the logical OR of two `Predicate` results, returning the result.
    fn or<B>(self, other: B) -> OrPredicate<Self, B, Item>
    where
        B: Predicate<Item>,
        Self: Sized,
    {
        OrPredicate::new(self, other)
    }

    /// Compute the logical NOT of a `Predicate`, returning the result.
    fn not(self) -> NotPredicate<Self, Item>
    where
        Self: Sized,
    {
        NotPredicate::new(self)
    }
}

impl<P, Item> PredicateBooleanExt<Item> for P
where
    P: Predicate<Item>,
    Item: ?Sized,
{
}

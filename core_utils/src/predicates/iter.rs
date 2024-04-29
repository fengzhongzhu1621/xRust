use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;

use crate::debug::DebugAdapter;
use crate::predicates::core::{
    default_find_case, Case, Palette, Parameter, Predicate,
    PredicateReflection, Product,
};

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// set, otherwise returns `false`.
///
/// Note that this implementation places the fewest restrictions on the
/// underlying `Item` type at the expense of having the least performant
/// implementation (linear search). If the type to be searched is `Hash + Eq`,
/// it is much more efficient to use `HashableInPredicate` and
/// `in_hash`. The implementation-specific predicates will be
/// deprecated when Rust supports trait specialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InPredicate<T>
where
    T: PartialEq + fmt::Debug,
{
    inner: DebugAdapter<Vec<T>>, // 数组类型
}

impl<T> InPredicate<T>
where
    T: Ord + fmt::Debug,
{
    /// Creates a new predicate that will return `true` when the given `variable` is
    /// contained with the set of items provided.
    pub fn sort(self) -> OrdInPredicate<T> {
        let mut items = self.inner.debug;
        items.sort();
        OrdInPredicate { inner: DebugAdapter::new(items) }
    }
}

impl<P, T> Predicate<P> for InPredicate<T>
where
    T: std::borrow::Borrow<P> + PartialEq + fmt::Debug,
    P: PartialEq + fmt::Debug + ?Sized,
{
    fn eval(&self, variable: &P) -> bool {
        self.inner.debug.iter().any(|x| x.borrow() == variable)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &P,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable).map(|case| {
            case.add_product(Product::new(
                "var",
                DebugAdapter::new(variable).to_string(),
            ))
        })
    }
}

impl<T> PredicateReflection for InPredicate<T>
where
    T: PartialEq + fmt::Debug,
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("values", &self.inner)];
        Box::new(params.into_iter())
    }
}

impl<T> fmt::Display for InPredicate<T>
where
    T: PartialEq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),
            palette.description("in"),
            palette.expected("values")
        )
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
pub fn in_iter<I, T>(iter: I) -> InPredicate<T>
where
    T: PartialEq + fmt::Debug,
    I: IntoIterator<Item = T>,
{
    InPredicate { inner: DebugAdapter::new(Vec::from_iter(iter)) }
}

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// set, otherwise returns `false`.
///
/// Note that this implementation requires `Item` to be `Ord`. The
/// `InPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// This is created by the `predicate::in_iter(...).sort` function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrdInPredicate<T>
where
    T: Ord + fmt::Debug,
{
    inner: DebugAdapter<Vec<T>>,
}

impl<P, T> Predicate<P> for OrdInPredicate<T>
where
    T: std::borrow::Borrow<P> + Ord + fmt::Debug,
    P: Ord + fmt::Debug + ?Sized,
{
    fn eval(&self, variable: &P) -> bool {
        self.inner.debug.binary_search_by(|x| x.borrow().cmp(variable)).is_ok()
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &P,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable).map(|case| {
            case.add_product(Product::new(
                "var",
                DebugAdapter::new(variable).to_string(),
            ))
        })
    }
}

impl<T> PredicateReflection for OrdInPredicate<T>
where
    T: Ord + fmt::Debug,
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("values", &self.inner)];
        Box::new(params.into_iter())
    }
}

impl<T> fmt::Display for OrdInPredicate<T>
where
    T: Ord + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),
            palette.description("in"),
            palette.expected("values")
        )
    }
}

/// Predicate that returns `true` if `variable` is a member of the pre-defined
/// `HashSet`, otherwise returns `false`.
///
/// Note that this implementation requires `Item` to be `Hash + Eq`. The
/// `InPredicate` uses a less efficient search algorithm but only
/// requires `Item` implement `PartialEq`. The implementation-specific
/// predicates will be deprecated when Rust supports trait specialization.
///
/// This is created by the `predicate::in_hash` function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
{
    inner: DebugAdapter<HashSet<T>>,
}

impl<P, T> Predicate<P> for HashableInPredicate<T>
where
    T: std::borrow::Borrow<P> + Hash + Eq + fmt::Debug,
    P: Hash + Eq + fmt::Debug + ?Sized,
{
    fn eval(&self, variable: &P) -> bool {
        self.inner.debug.contains(variable)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &P,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable).map(|case| {
            case.add_product(Product::new(
                "var",
                DebugAdapter::new(variable).to_string(),
            ))
        })
    }
}

impl<T> PredicateReflection for HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("values", &self.inner)];
        Box::new(params.into_iter())
    }
}

impl<T> fmt::Display for HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),
            palette.description("in"),
            palette.expected("values")
        )
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// contained with the set of items provided.
pub fn in_hash<I, T>(iter: I) -> HashableInPredicate<T>
where
    T: Hash + Eq + fmt::Debug,
    I: IntoIterator<Item = T>,
{
    HashableInPredicate { inner: DebugAdapter::new(HashSet::from_iter(iter)) }
}

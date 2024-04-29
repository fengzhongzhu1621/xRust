use std::fmt;
use std::marker::PhantomData;

use crate::predicates::core::{
    default_find_case, Case, Palette, Predicate,
    PredicateReflection,
};

/// Predicate that wraps a function over a reference that returns a `bool`.
/// This type is returned by the `predicate::function` function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
    T: ?Sized,
{
    function: F,
    name: &'static str,
    _phantom: PhantomData<T>,
}

unsafe impl<F, T> Send for FnPredicate<F, T>
where
    F: Send + Fn(&T) -> bool,
    T: ?Sized,
{
}

unsafe impl<F, T> Sync for FnPredicate<F, T>
where
    F: Sync + Fn(&T) -> bool,
    T: ?Sized,
{
}

impl<F, T> FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
    T: ?Sized,
{
    /// Provide a descriptive name for this function.
    pub fn fn_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
}

impl<F, T> Predicate<T> for FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
    T: ?Sized,
{
    /// 执行函数
    fn eval(&self, variable: &T) -> bool {
        (self.function)(variable)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &T,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable)
    }
}

impl<F, T> PredicateReflection for FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
    T: ?Sized,
{
}

impl<F, T> fmt::Display for FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(f, "{}({})", palette.description(self.name), palette.var("var"),)
    }
}

pub fn function<F, T>(function: F) -> FnPredicate<F, T>
where
    F: Fn(&T) -> bool,
    T: ?Sized,
{
    FnPredicate { function, name: "fn", _phantom: PhantomData }
}

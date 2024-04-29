use crate::predicates::core::{
    Case, Child, Predicate, PredicateReflection, Product,
};
use std::ffi;
use std::fmt;
use std::str;

/// Predicate adaper that converts a `str` predicate to byte predicate.
///
/// This is created by `pred.from_utf8()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utf8Predicate<P>
where
    P: Predicate<str>,
{
    pub(crate) p: P,
}

impl<P> Predicate<ffi::OsStr> for Utf8Predicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &ffi::OsStr) -> bool {
        variable.to_str().map(|s| self.p.eval(s)).unwrap_or(false)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &ffi::OsStr,
    ) -> Option<Case<'a>> {
        let var_str = variable.to_str();
        match (expected, var_str) {
            (_, Some(var_str)) => {
                self.p.find_case(expected, var_str).map(|child| {
                    child.add_product(Product::new(
                        "var as str",
                        var_str.to_owned(),
                    ))
                })
            }
            (true, None) => None,
            (false, None) => {
                Some(Case::new(Some(self), false).add_product(Product::new(
                    "error",
                    "Invalid UTF-8 string",
                )))
            }
        }
    }
}

impl<P> Predicate<[u8]> for Utf8Predicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, variable: &[u8]) -> bool {
        str::from_utf8(variable).map(|s| self.p.eval(s)).unwrap_or(false)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &[u8],
    ) -> Option<Case<'a>> {
        let var_str = str::from_utf8(variable);
        match (expected, var_str) {
            (_, Ok(var_str)) => {
                self.p.find_case(expected, var_str).map(|child| {
                    child.add_product(Product::new(
                        "var as str",
                        var_str.to_owned(),
                    ))
                })
            }
            (true, Err(_)) => None,
            (false, Err(err)) => Some(
                Case::new(Some(self), false)
                    .add_product(Product::new("error", err)),
            ),
        }
    }
}

impl<P> PredicateReflection for Utf8Predicate<P>
where
    P: Predicate<str>,
{
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params = vec![Child::new("predicate", &self.p)];
        Box::new(params.into_iter())
    }
}

impl<P> fmt::Display for Utf8Predicate<P>
where
    P: Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.p.fmt(f)
    }
}
